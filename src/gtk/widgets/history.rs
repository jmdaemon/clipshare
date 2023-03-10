use gtk::prelude::{
    ButtonExt,
    BoxExt,
    OrientableExt,
    WidgetExt,
};

use relm4::{
    factory::{FactoryView, FactoryVecDeque},
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    ComponentSender,
    ComponentParts,
    FactorySender,
    SimpleComponent,
};

#[derive(Debug)]
pub struct HistoryEntry {
    pub index: usize,
    pub last_copied: String,
}

#[derive(Debug)]
pub enum HistoryEntryInput {}

#[derive(Debug)]
pub enum HistoryEntryOutput {}

pub struct HistoryEntryInit {}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryEntry {
    type Init = HistoryEntry;
    type Input = HistoryEntryInput;
    type Output = HistoryEntryOutput;
    type CommandOutput = ();
    type ParentInput = HistoryInput;
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBox {
            set_selection_mode: gtk::SelectionMode::Single,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                set_hexpand: true,
                gtk::Label {
                    #[watch]
                    set_label: &self.index.to_string(),
                    set_width_chars: 8,
                    set_xalign: 0.6,
                },
                gtk::Label {
                    #[watch]
                    set_label: &self.last_copied,
                    set_width_chars: 128,
                    set_xalign: 0.0,
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
    }

    fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        init
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


#[derive(Debug)]
pub struct HistoryModel {
    pub history: FactoryVecDeque<HistoryEntry>,
}

#[derive(Debug)]
pub enum HistoryInput {}

#[derive(Debug)]
pub enum HistoryOutput {}

#[relm4::component(pub)]
impl SimpleComponent for HistoryModel {
    type Input = HistoryInput;
    type Output = HistoryOutput;
    type Init = ();

    view! {
        #[root]
        gtk::ScrolledWindow {
            #[local_ref]
            history_box -> gtk::ListBox,
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let mut history = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());

        // Test out adding a few entries
        let clipboard = vec![
            "This".to_owned(),
            "Will".to_owned(),
            "Be".to_owned(),
            "Copied".to_owned(),
        ];

        let mut index: usize = 1;
        clipboard.into_iter().for_each(|line| {
            let entry = HistoryEntry { index, last_copied: line };
            history.guard().push_front(entry);
            index += 1;
        });

        let model = HistoryModel { history };
        let history_box = model.history.widget();

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {

        }
    }
}
