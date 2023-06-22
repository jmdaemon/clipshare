use super::history::HistoryViewModel;
use relm4::{
    gtk,
    ComponentSender,
    ComponentParts,
    Controller,
    Component,
};

#[derive(Debug)]
pub struct DeviceViewModel {
    pub name: String, // This should be replaced with a wrapper struct of Device
    pub history: Controller<HistoryViewModel>,
}

#[derive(Debug)]
pub struct DeviceViewWidgets {
    pub history_widget: gtk::ScrolledWindow,
}

pub fn create_label(name: &str) -> gtk::Label {
    gtk::Label::new(Some(name))
}

// TODO: Since the HistoryModel is difficult to work with, this will have to do for now
// Later on we'll replace this with a proper DeviceViewModel constructor
impl DeviceViewModel {
    pub fn new(name: String) -> Self {
        let history_builder = HistoryViewModel::builder();
        //let history_widget = history_builder.widget().to_owned();
        let history = history_builder.launch(()).detach();
        Self { name, history }
    }
}

//#[relm4::component(pub)]
impl Component for DeviceViewModel {
    type Input = ();
    type Output = ();
    type Init = String;
    type Root = gtk::Box;
    type Widgets = DeviceViewWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        gtk::Box::new(gtk::Orientation::Vertical, 6)
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        // TODO: Use the builder directly to attach the object to the screen?
        let history_builder = HistoryViewModel::builder();
        let history_widget = history_builder.widget().to_owned();
        let history = history_builder.launch(()).detach();

        let model = DeviceViewModel { name: init, history };
        let widgets = DeviceViewWidgets { history_widget };

        ComponentParts { model, widgets }
    }
}
