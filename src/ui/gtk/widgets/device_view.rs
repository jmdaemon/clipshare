use super::history_view::HistoryViewModel;
use relm4::{
    gtk::{self, traits::BoxExt},
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

impl Component for DeviceViewModel {
    type Input = ();
    type Output = ();
    type Init = String;
    type Root = gtk::Box;
    type Widgets = DeviceViewWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        info!("Constructing DeviceView");
        let history_builder = HistoryViewModel::builder();
        let history_widget = history_builder.widget().to_owned();
        let history = history_builder.launch(()).detach();

        let model = DeviceViewModel { name: init, history };
        let widgets = DeviceViewWidgets { history_widget };

        // Add widgets to main view
        root.append(&widgets.history_widget);

        ComponentParts { model, widgets }
    }
}
