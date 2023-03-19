use super::history::HistoryModel;
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

pub type DevicePool = VecDeque<DeviceModel>;

#[derive(Debug)]
pub struct DeviceModel {
    pub name: String,
    pub history_widget: gtk::ScrolledWindow,
    pub history: Controller<HistoryModel>,
}

#[derive(Debug)]
pub struct DeviceView {
    pub device_pool: DevicePool,
}

#[derive(Debug)]
pub enum DeviceViewInput {
    AddDevice(String),
    RemoveDevice(String),
    ReorderDevice(String, u32),
}

pub fn get_hostname() -> String {
        hostname::get().expect("Could not get hostname").to_str().unwrap().to_string()
}

pub fn create_device_tab_title(name: &str) -> gtk::Label {
    gtk::Label::new(Some(name))
}

pub fn create_device(name: String) -> DeviceModel {
    let history_builder = HistoryModel::builder();
    let history_widget = history_builder.widget().to_owned();
    let history = history_builder.launch(()).detach();
    DeviceModel { name, history_widget, history }
}

pub fn add_device(name: String, root: &gtk::Notebook) -> DeviceModel {
    let tab_title = create_device_tab_title(&name);
    let device = create_device(name);

    root.append_page(&device.history_widget, Some(&tab_title));
    device
}

pub fn remove_device(name: String, root: &gtk::Notebook, device_pool: &DevicePool) {
    for (index, dev) in device_pool.iter().enumerate() {
        if dev.name == name {
            root.remove_page(Some(index as u32));
        }
    }
}

pub fn reorder_device(name: String, root: &gtk::Notebook, device_pool: &DevicePool, to: u32) {
    for dev in device_pool.iter() {
        if dev.name == name {
            root.reorder_child(&dev.history_widget, Some(to));
        }
    }
}

#[relm4::component(pub)]
impl Component for DeviceView {
    type Input = DeviceViewInput;
    type Output = ();
    type Init = ();
    type CommandOutput = ();

    view! {
           #[root]
            gtk::Notebook {
                set_hexpand: true,
                set_vexpand: true,
            }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let device = add_device(get_hostname(), root);
        let device2 = add_device("Test Name".to_owned(), root);

        let mut device_pool = VecDeque::new();
        device_pool.push_back(device);
        device_pool.push_back(device2);

        //remove_device(get_hostname(), root, &device_pool);
        reorder_device(get_hostname(), root, &device_pool, 1);

        let model = DeviceView { device_pool };

        let widgets = view_output!();


        ComponentParts { model, widgets }
    }
    
    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root
    ) {
        match message {
            DeviceViewInput::AddDevice(device_name) => {
                let device = add_device(device_name, root);
                self.device_pool.push_back(device);
            },
            DeviceViewInput::RemoveDevice(device_name) => {
                remove_device(device_name, root, &self.device_pool);
            },
            DeviceViewInput::ReorderDevice(device_name, to) => {
                reorder_device(device_name, root, &self.device_pool, to);
            }
        }
    }
}
