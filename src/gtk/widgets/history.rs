use gtk::prelude::{
    BoxExt,
    ButtonExt,
    OrientableExt,
    WidgetExt,
    //FrameExt,
};
//use gdk4::{
    //Cursor::
    
//};

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

#[derive(Debug)]
pub enum HistoryEntryInput {}

#[derive(Debug)]
pub enum HistoryEntryOutput {}

pub struct HistoryEntryInit {}

//pub fn index(i: DynamicIndex) -> String {
    //String::from(self.index.current_index() + 1)
//}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryEntry {
    type Init = String;
    type Input = HistoryEntryInput;
    type Output = HistoryEntryOutput;
    type CommandOutput = ();
    type ParentInput = HistoryInput;
    //type ParentWidget = gtk::Box;
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,
            set_hexpand: true,
                gtk::Label {
                    #[watch]
                    set_label: &(self.index.current_index() + 1).to_string(),
                    set_width_chars: 8,
                    set_xalign: 0.6,
                },

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
            //#[name="lb_entries"]
            gtk::ListBox {
                #[local_ref]
                //history_box -> gtk::Box,
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
        //let mut history = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());

        // Test out adding a few entries
        let clipboard = vec![
            "This".to_owned(),
            "Will".to_owned(),
            "Be".to_owned(),
            "Copied".to_owned(),
        ];

        //let mut index: usize = 1;
        //clipboard.into_iter().for_each(|line| {
        //let history = clipboard.into_iter().for_each(|line| {
        clipboard.into_iter().for_each(|line| {
            //history.guard().push_front(line);
            //history.guard().push_back(line);

            /*
            let len = history.len();
            let last = history.get(len);
            if let Some(last) = last {
                history.guard().swap(len, 0);
                //last.index.to_owned + 1;
            }
            */

            let index = history.guard().push_back(line);
            history.guard().move_front(index.current_index());

            //let entry = HistoryEntry { index, last_copied: line };
            //history.guard().push_front(entry);
            //index += 1;
        });
        //}).rev().collect();
        //history.iter().rev()
        //let history = history.iter().map(|entry: &HistoryEntry| entry.to_owned()).rev().collect();

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

        }
    }
}
