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

use super::history::{HistoryModel, HistoryModelWidgets};

#[derive(Debug)]
pub struct DeviceModel {
    pub name: String,
    //pub history: Vec<String>,
    //pub history_widget: gtk::ScrolledWindow,
    //pub history_model_widget: Controller<HistoryModel>,
    //pub history_builder: ComponentBuilder<HistoryModel>,
    pub history_widget: gtk::ScrolledWindow,
    pub history: Controller<HistoryModel>,
}

#[derive(Debug)]
pub struct DeviceView {
    //pub view: gtk::Notebook,
    pub device_pool: VecDeque<DeviceModel>,
}

#[derive(Debug)]
pub enum DeviceViewInput {
    AddDevice(String),
    RemoveDevice(String),
    ReorderDevice(String, DynamicIndex, DynamicIndex),
}

//impl DeviceView {
    //pub fn create_device(device_name: String) -> (DeviceModel, gtk::Label) {
        //let tab_title = gtk::Label::new(Some(&device_name));

        //let history_model = HistoryModel::builder();
        //let history_widget = history_model.widget().to_owned();

        //let device = DeviceModel { name: device_name, history: vec![], history_widget };
        //(device, tab_title)
    //}
//}

//pub fn create_history_widget() {
    //let history_model = HistoryModel::builder();
    //let history_widget = history_model.widget().to_owned();
    //let history_model_widget = history_model.launch(()).detach();
//}

pub fn get_hostname() -> String {
        hostname::get().expect("Could not get hostname").to_str().unwrap().to_string()
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

        let name = get_hostname();
        let tab_title = gtk::Label::new(Some(&name));

        let history_builder = HistoryModel::builder();
        let history_widget = history_builder.widget().to_owned();
        let history = history_builder.launch(()).detach();
        let device = DeviceModel { name, history_widget, history };

        //let history_widget = history_model.widget().to_owned();

        //let history_model_widget = history_model.launch(()).detach();
        //let history = history_builder.launch(()).detach();
        //&device.history_builder.launch(()).detach();

        //root.append_page(&history, Some(&tab_title));
        //root.append_page(&device.history_builder.widget().to_owned(), Some(&tab_title));
        root.append_page(&device.history_widget.to_owned(), Some(&tab_title));

        let mut device_pool = VecDeque::new();
        device_pool.push_back(device);

        //self.view.append_page(&device.history_widget, Some(&tab_title));

        let model = DeviceView { device_pool };
        //for dev in device_pool {
        //}


        //

        //let view_widget = model.view;


        let widgets = view_output!();


        ComponentParts { model, widgets }
    }
    
    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root) {
        match message {
        DeviceViewInput::AddDevice(device_name) => {
                // Add to the view
            /*
                let tab_title = gtk::Label::new(Some(&device_name));

                let history_model = HistoryModel::builder();
                let history_widget = history_model.widget().to_owned();

                let history_model_widget = history_model.launch(()).detach();
                let device = DeviceModel { name: device_name, history: vec![], history_model_widget, history_widget };

                //let history_model = HistoryModel::builder()
                    //.launch(()).detach();
                //let history_widget = history_model.widget();

                //self.view.append_page(&history_widget, Some(&tab_title));

                //self.view.append_page(&history_model, Some(&tab_title));

                //self.view.append_page(&device.history_widget, Some(&tab_title));
                root.append_page(&device.history_widget, Some(&tab_title));

                //self.view.page(&history_widget).set_tab_expand
                // Add to our managed pool
            */
            },
            DeviceViewInput::RemoveDevice(device_name) => { },
            DeviceViewInput::ReorderDevice(device_name, from, to) => { }
        }
    }
}
