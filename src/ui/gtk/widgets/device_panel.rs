use super::{history::HistoryModel, device_view::{DeviceView, DeviceModel}};
use std::collections::VecDeque;
use gtk::prelude::{
    BoxExt,
    ButtonExt,
    OrientableExt,
    WidgetExt,
};
use relm4::{
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts,
    prelude::DynamicIndex,
    WidgetRef,
    RelmWidgetExt,
    Controller,
    Component, ComponentBuilder,
};

// Types
pub type DeviceViews = VecDeque<DeviceModel>;

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
    ReorderDevice(String, u32),
}

// Functions
pub fn create_device_tab_title(name: &str) -> gtk::Label {
    gtk::Label::new(Some(name))
}

pub fn create_device(name: String) -> DeviceModel {
    let history_builder = HistoryModel::builder();
    let history_widget = history_builder.widget().to_owned();
    let history = history_builder.launch(()).detach();
    DeviceModel { name, history_widget, history }
}

//pub fn add_device(name: String, root: &gtk::Notebook) -> DeviceModel {
    //let tab_title = create_device_tab_title(&name);
    //let device = create_device(name);

    //root.append_page(&device.history_widget, Some(&tab_title));
    //device
//}

//pub fn remove_device(name: String, root: &gtk::Notebook, device_pool: &DevicePool) {
    //for (index, dev) in device_pool.iter().enumerate() {
        //if dev.name == name {
            //root.remove_page(Some(index as u32));
        //}
    //}
//}

//pub fn reorder_device(name: String, root: &gtk::Notebook, device_pool: &DevicePool, to: u32) {
    //for dev in device_pool.iter() {
        //if dev.name == name {
            //root.reorder_child(&dev.history_widget, Some(to));
        //}
    //}
//}

pub trait DeviceNotebookActions {
    fn inner(&self) -> &gtk::Notebook;
    fn add_device(&self, name: String);
    fn remove_device(&self, name: String, device_views: &DeviceViews);
    fn reorder_device(&self, name: String, device_views: &DeviceViews, to: u32);
}

impl DeviceNotebookActions for DeviceNotebook {
    fn inner(&self) -> &gtk::Notebook { self.0.as_ref() }

    fn add_device(&self, name: String) {
        let tab_title = create_device_tab_title(&name);
        let device = create_device(name);
        self.inner().append_page(&device.history_widget, Some(&tab_title));
    }

    fn remove_device(&self, name: String, device_views: &DeviceViews) {
        device_views.iter().enumerate().for_each( |(index, dev)| {
            if dev.name == name {
                self.inner().remove_page(Some(index as u32));
            }
        });
    }

    fn reorder_device(&self, name: String, device_views: &DeviceViews, to: u32) {
        device_views.iter().for_each(|dev| {
            if dev.name == name {
                self.inner().reorder_child(&dev.history_widget, Some(to));
            }
        });
    }
}

// Component
//#[relm4::component(pub)]
impl Component for DevicePanelModel {
    type Input = DevicePanelAction;
    type Output = ();
    type Init = ();
    type Widgets = DevicePanelWidgets;
    type Root = gtk::Notebook;
    type CommandOutput = ();

    //view! {
        //#[local_ref]
        //device_notebook.inner() -> gtk::Notebook {
        //}

        //#[root]
        //gtk::Notebook {
              //set_hexpand: true,
              //set_vexpand: true,
        //}
    //}

    fn init_root() -> Self::Root {
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
        // Create GtkNotebook
        //let notebook = gtk::Notebook::new();
        //notebook.set_hexpand(true);
        //notebook.set_vexpand(true);

        //let device_notebook = DeviceNotebook(notebook);
        let device_notebook = DeviceNotebook(root.to_owned());

        // Create model
        let model = DevicePanelModel { device_views };
        let widgets = DevicePanelWidgets { device_notebook };

        ComponentParts { model, widgets }
    }

    //fn update(
        //&mut self,
        //message: Self::Input,
        //sender: ComponentSender<Self>,
    //) {
        //match message {
            //DevicePanelAction::AddDevice(device_name) => {
                //// TODO: Device should be an already constructed object here
                //// TODO: This doesn't seem to actually add another
                ////let device = add_device(device_name, root);
                ////let device = add_device(device_name, root);
                ////(*self).add_device(device_name, root);
                ////let device = add_device(device_name, root);
                //root.add_device(device_name);
                //self.device_views.push_back(device);
            //},
            //DevicePanelAction::RemoveDevice(device_name) => {
                //remove_device(device_name, root, &self.device_pool);
            //},
            //DevicePanelAction::ReorderDevice(device_name, to) => {
                //reorder_device(device_name, root, &self.device_pool, to);
            //}
        //}
    //}
    
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
                // TODO: Device should be an already constructed object here
                // TODO: This doesn't seem to actually add another
                //let device = add_device(device_name, root);
                //let device = add_device(device_name, root);
                //(*self).add_device(device_name, root);
                //let device = add_device(device_name, root);
                //root.add_device(device_name);
                //self.device_views.push_back(device);
                device_notebook.add_device(device_name);
            },
            DevicePanelAction::RemoveDevice(device_name) => {
                //root.remove_device(device_name, root, &self.device_views);
                device_notebook.remove_device(device_name, &self.device_views);
            },
            DevicePanelAction::ReorderDevice(device_name, to) => {
                //reorder_device(device_name, root, &self.device_views, to);
                device_notebook.reorder_device(device_name, &self.device_views, to);
            }
        }
    }
}
