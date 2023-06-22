use crate::connect::client::ClientPool;

use super::device_view::{DeviceViewModel, create_label};
use std::collections::VecDeque;
use gtk::prelude::WidgetExt;
use relm4::{
    gtk,
    ComponentSender,
    ComponentParts,
    Component,
};

// Types
pub type DeviceViews = VecDeque<DeviceViewModel>;

#[derive(Debug)]
pub struct DevicePanelModel {
    pub device_views: DeviceViews,
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
    fn add_device(&self, name: &str);
    fn remove_device(&self, name: String, device_views: &DeviceViews);
    fn reorder_device(&self, name: String, device_views: &DeviceViews, from: u32, to: u32);
}

impl DeviceNotebookActions for DeviceNotebook {
    fn inner(&self) -> &gtk::Notebook { self.0.as_ref() }

    fn add_device(&self, name: &str) {
        let tab_title = create_label(&name);
        //let device = create_device(name);
        //let device = DeviceViewModel::new(name);
        let device_builder = DeviceViewModel::builder();
        let device_widget = device_builder.widget();
        self.inner().append_page(device_widget, Some(&tab_title));
        //let a = device_builder.launch(()).detach();
    }

    fn remove_device(&self, name: String, device_views: &DeviceViews) {
        device_views.iter().enumerate().for_each( |(index, dev)| {
            if dev.name == name {
                self.inner().remove_page(Some(index as u32));
            }
        });
    }

    fn reorder_device(&self, name: String, device_views: &DeviceViews, from: u32, to: u32) {
        //let c = self.0.page();
        //self.0.pages().into_iter().for_each(|(a)| {
            //let b: gtk::ScrolledWindow = a.unwrap().();
        //});
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

//pub fn create_device(name: String) -> DeviceModel {
    //let history_builder = HistoryViewModel::builder();
    //let history_widget = history_builder.widget().to_owned();
    //let history = history_builder.launch(()).detach();
    //DeviceModel { name, history_widget, history }
//}

// Component
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
        let device_notebook = DeviceNotebook(root.to_owned());
        // Add the device views here
        let clients = init.clients.lock().unwrap();
        clients.iter().for_each(|client| {
            let client_unlock = client.device.lock().unwrap();
            let name = client_unlock.name.as_ref();
            device_notebook.add_device(name);
        });

        // Create model
        let model = DevicePanelModel { device_views };
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
                device_notebook.add_device(&device_name);
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
