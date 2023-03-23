use gtk::prelude::{
    BoxExt,
    ButtonExt,
    OrientableExt,
    WidgetExt,
    //FrameExt,
};

use relm4::{
    factory::{FactoryView, FactoryVecDeque},
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    ComponentSender,
    ComponentParts,
    FactorySender,
    SimpleComponent,
    RelmWidgetExt,
};

#[derive(Debug)]
pub struct HistoryEntry {
    pub index: DynamicIndex,
    pub last_copied: String,
}

//#[derive(Debug)]
//pub enum HistoryEntryInput {
    //AddEntry,
//}

#[derive(Debug)]
pub enum HistoryEntryOutput {
    CopyEntry(DynamicIndex),
    DeleteEntry(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryEntry {
    type Init = String;
    //type Input = HistoryEntryInput;
    type Input = ();
    type Output = HistoryEntryOutput;
    type CommandOutput = ();
    type ParentInput = HistoryInput;
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        root = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,
            set_hexpand: true,

            #[name(lbl_index)]
            gtk::Label {
                #[watch]
                set_label: &(self.index.current_index() + 1).to_string(),
                set_width_chars: 8,
                set_xalign: 0.6,
            },

            #[name(lbl_history_entry)]
            gtk::Label {
                set_css_classes: &["clipboard-entry"],
                set_can_target: true,

                #[watch]
                set_label: &self.last_copied,
                set_width_chars: 128,
                set_xalign: 0.00,
                //connect_cursor_notify[sender] => move |_| {
                    //println!("Changed cursor");
                    //gtk::Widget::set_cursor(&self, gdk::Cursor::);
                    //gtk::Window::new().set_cursor(gtk::Widget::set_cursor(&self, cursor));
                //}
            },

            #[name(btn_copy)]
            gtk::Button {
                set_height_request: 24,
                set_label: "Copy",
                connect_clicked[sender, index] => move |_| {
                    sender.output(HistoryEntryOutput::CopyEntry(index.clone()))
                }
            },

            #[name(btn_delete)]
            gtk::Button {
                set_height_request: 24,
                set_label: "Delete",
                connect_clicked[sender, index] => move |_| {
                    sender.output(HistoryEntryOutput::DeleteEntry(index.clone()))
                }
                //connect_clicked[sender, index] => move |_| {
                    //let index = self.index.current_index();
                    //sender.output(HistoryInput::DeleteEntry(index));
                    //sender.output(HistoryInput::DeleteEntry(0));
                    //sender.input(HistoryEntryInput::DeleteEntry);
                //}
            },
        }
    }

    fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        Self {
            last_copied: init,
            index: index.to_owned()
        }
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn output_to_parent_input(output: Self::Output) -> Option<HistoryInput> {
        Some(match output {
            HistoryEntryOutput::DeleteEntry(index) => HistoryInput::DeleteEntry(index),
            HistoryEntryOutput::CopyEntry(index) => HistoryInput::CopyEntry(index),
        })
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        //match message {
            //HistoryEntryInput::AddEntry => {

            //},

            /*
            HistoryEntryInput::CopyEntry => {
                // Copy the clipboard to the current device contents
            }
            HistoryEntryInput::DeleteEntry => {
                // Send the index to the main factory widget
                // In the main factory widget, delete the current entry
                // Set the current index to the index of the deleted widget?
                let index = self.index.current_index();
                sender.output(HistoryInput::DeleteEntry(index));
            }
            */
        //}
    }

    /*
    fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        let output = match output {};
        Some(output)
    }
    */
}

#[derive(Debug)]
pub struct HistoryModel {
    pub history: FactoryVecDeque<HistoryEntry>,
}

#[derive(Debug)]
pub enum HistoryInput {
    CopyEntry(DynamicIndex),
    DeleteEntry(DynamicIndex),
}

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
            gtk::ListBox {
                #[local_ref]
                history_box -> gtk::ListBox,
            }
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

        clipboard.into_iter().for_each(|line| {
            let index = history.guard().push_back(line);
            history.guard().move_front(index.current_index());
        });

        let model = HistoryModel { history };

        //model.history.iter() {
        //}
        let history_box = model.history.widget();
        relm4::set_global_css_from_file("src/gtk/widgets/history.css");

        let widgets = view_output!();

        //let append_entry = |lb_entries: &gtk::ListBox| {
            //for entry in model.history.iter() {
                //lb_entries.append(&entry.init_root());
                ////let a = entry.init_root();
                ////lb_entries.append(entry);
            //}
            //lb_entries
        //};

        //lb_entries = append_entry(&lb_entries).to_owned();
        //for entry in model.history.iter() {
            //lb_entries.append(&entry.init_root());
            ////let a = entry.init_root();
            ////lb_entries.append(entry);
        //}
        
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            HistoryInput::DeleteEntry(index) => {
                println!("Received DeleteEntry");
                self.history.guard().remove(index.current_index());

                //let child = self.history.get(position).unwrap().init_root();
                //let child = self.history.get(position).unwrap().init_root();
                //self.history.widget().remove(&child);
                
                //let last = self.history.len();
                //self.history.guard().pop_back();
                //self.history.widget().remove(child)

                /*
                self.history.guard().move_to(position, last);
                //self.history.guard().move_to(position, 0);
                self.history.guard().pop_back();
                */
            }
            HistoryInput::CopyEntry(index) => {
                // Copy the thing
            }
        }
    }
}
