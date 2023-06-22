use crate::connect::client::ClientPool;

use super::device_view::{DeviceViewModel, create_label};
use std::collections::VecDeque;
use gtk::prelude::WidgetExt;
use relm4::{
    gtk,
    ComponentSender,
    ComponentParts,
    Component, Controller,
};

// Types
pub type DeviceViews = VecDeque<DeviceViewModel>;
pub type DeviceViewControllers = VecDeque<Controller<DeviceViewModel>>;

#[derive(Debug)]
pub struct DevicePanelModel {
    pub device_views: DeviceViews,
    pub device_views_controllers: DeviceViewControllers,
}

#[derive(Debug)]
pub struct DeviceNotebook(gtk::Notebook);

#[derive(Debug)]
pub struct DevicePanelWidgets {
    pub device_notebook: DeviceNotebook,
}

#[derive(Debug)]
pub enum DevicePanelAction {
    AddDevice(String),
    RemoveDevice(String),
    ReorderDevice(String, u32, u32),
}

// We create a newtype struct around GtkNotebook in order to more easily manage devices
pub trait DeviceNotebookActions {
    fn inner(&self) -> &gtk::Notebook;
    fn add_device(&self, name: &str, controllers: &mut DeviceViewControllers);
    fn remove_device(&self, name: String, device_views: &DeviceViews);
    fn reorder_device(&self, name: String, device_views: &DeviceViews, from: u32, to: u32);
}

impl DeviceNotebookActions for DeviceNotebook {
    fn inner(&self) -> &gtk::Notebook { self.0.as_ref() }

    fn add_device(&self, name: &str, controllers: &mut DeviceViewControllers) {
        let tab_title = create_label(name);

        let device_builder = DeviceViewModel::builder();

        let device_widget = device_builder.widget();
        self.inner().append_page(device_widget, Some(&tab_title));

        let controller = device_builder.launch(name.to_owned()).detach();
        controllers.push_back(controller);
    }

    fn remove_device(&self, name: String, device_views: &DeviceViews) {
        device_views.iter().enumerate().for_each( |(index, dev)| {
            if dev.name == name {
                self.inner().remove_page(Some(index as u32));
            }
        });
    }

    fn reorder_device(&self, name: String, device_views: &DeviceViews, from: u32, to: u32) {
        device_views.iter().for_each(|dev| {
            if dev.name == name {
                let maybe_history_widget = self.inner().nth_page(Some(from));
                if let Some(history_widget) = maybe_history_widget {
                    self.inner().reorder_child(&history_widget, Some(to));
                }
            }
        });
    }
}

impl Component for DevicePanelModel {
    type Input = DevicePanelAction;
    type Output = ();
    type Init = ClientPool;
    type Widgets = DevicePanelWidgets;
    type Root = gtk::Notebook;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        // Create GtkNotebook
        let notebook = gtk::Notebook::new();
        notebook.set_hexpand(true);
        notebook.set_vexpand(true);
        notebook
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let device_views = VecDeque::new();
        let mut device_views_controllers = VecDeque::new();
        let device_notebook = DeviceNotebook(root.to_owned());
        // Add the device views here
        info!("Constructing DevicePanel");
        let clients = init.clients.lock().unwrap();
        clients.iter().for_each(|client| {
            info!("Creating device");
            let client_unlock = client.device.lock().unwrap();
            let name = client_unlock.name.as_ref();
            device_notebook.add_device(name, &mut device_views_controllers);
        });

        // Create model
        let model = DevicePanelModel { device_views, device_views_controllers };
        let widgets = DevicePanelWidgets { device_notebook };

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root
    ) {
        let device_notebook = &widgets.device_notebook;
        match message {
            DevicePanelAction::AddDevice(device_name) => {
                device_notebook.add_device(&device_name, &mut self.device_views_controllers);
                //self.device_views.push_back(device);
            },
            DevicePanelAction::RemoveDevice(device_name) => {
                device_notebook.remove_device(device_name, &self.device_views);
            },
            DevicePanelAction::ReorderDevice(device_name, from, to) => {
                device_notebook.reorder_device(device_name, &self.device_views, from, to);
            }
        }
    }
}
