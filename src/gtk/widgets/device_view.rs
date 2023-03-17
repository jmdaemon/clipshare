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
    Component,
};

use super::history::{HistoryModel, HistoryModelWidgets};

#[derive(Debug)]
pub struct DeviceModel {
    pub name: String,
    pub history: Vec<String>,
    pub history_model_widget: Controller<HistoryModel>,
    pub history_widget: gtk::ScrolledWindow,
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

#[relm4::component(pub)]
//impl SimpleComponent for DeviceView {
impl Component for DeviceView {
    type Input = DeviceViewInput;
    type Output = ();
    //type Init = DeviceModel;
    type Init = ();
    type CommandOutput = ();

    view! {
        //#[root]
        //gtk::Box {
            //#[local_ref]
            //view_widget -> gtk::Notebook {
            //self.model.view -> gtk::Notebook {
            //view = self.model.view {
            //model.view -> gtk::Notebook {
            //view_widget -> gtk::Notebook {
            //view_widget -> gtk::Notebook {
            #[root]
            gtk::Notebook {
                set_hexpand: true,
                set_vexpand: true,
            }
        //}
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        // Add the current device
        //let device = DeviceModel {
            //name: hostname::get().unwrap().to_str().unwrap().to_owned(),
            //history: vec![],
        //};
        //let mut device_pool = VecDeque::new();
        //device_pool.push_back(init);

        //let view = gtk::Notebook::new();

        //let model = DeviceView { view, device_pool };

        //
        let device_name = hostname::get().expect("Could not get hostname").to_str().unwrap().to_string();
        let tab_title = gtk::Label::new(Some(&device_name));

        let history_model = HistoryModel::builder();
        let history_widget = history_model.widget().to_owned();

        let history_model_widget = history_model.launch(()).detach();

        let device = DeviceModel { name: device_name, history: vec![], history_model_widget, history_widget,  };

        let mut device_pool = VecDeque::new();
        root.append_page(&device.history_widget, Some(&tab_title));
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
            },
            DeviceViewInput::RemoveDevice(device_name) => { },
            DeviceViewInput::ReorderDevice(device_name, from, to) => { }
        }
    }

    //fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        /*
    fn update(
        &mut self,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root
    ) {
        match message {
            /*
            DeviceViewInput::AddDevice(device_name) => {
                // Add to the view
                let tab_title = gtk::Label::new(Some(&device_name));

                let history_model = HistoryModel::builder();
                let history_widget = history_model.widget().to_owned();

                let device = DeviceModel { name: device_name, history: vec![], history_widget };

                //let history_model = HistoryModel::builder()
                    //.launch(()).detach();
                //let history_widget = history_model.widget();

                //self.view.append_page(&history_widget, Some(&tab_title));

                //self.view.append_page(&history_model, Some(&tab_title));

                self.view.append_page(&device.history_widget, Some(&tab_title));

                //self.view.page(&history_widget).set_tab_expand
                // Add to our managed pool
            },
            */
            DeviceViewInput::RemoveDevice(device_name) => {
                //self.page.
                // Remove from the view
                // Remove from our managed pool
            },
            DeviceViewInput::ReorderDevice(device_name, from, to) => {
            }
            _ => {}
        }
    }
    */
}
