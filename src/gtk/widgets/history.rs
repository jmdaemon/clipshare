use gtk::prelude::{
    ButtonExt,
    BoxExt,
    OrientableExt,
    WidgetExt,
};

use relm4::{
    factory::FactoryView,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

pub struct HistoryModel {
    pub last_copied: String,
}

#[derive(Debug)]
pub enum HistoryInput {}

#[derive(Debug)]
pub enum HistoryOutput {}

pub struct HistoryInit {}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryModel {
    type ParentWidget = gtk::Box;
    type ParentInput = ();
    type Input = HistoryInput;
    type Output = HistoryOutput;
    type Init = String;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,

            gtk::Label {
                #[watch]
                set_label: &self.last_copied,
                set_width_chars: 64,
            },
            gtk::Button {
                set_height_request: 24,
                set_label: "Copy"
            },
            gtk::Button {
                set_height_request: 24,
                set_label: "Delete"
            },
        }
    }

    fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        Self {
            last_copied: init
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {}
    }

    fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        let output = match output {};
        Some(output)
    }
}
