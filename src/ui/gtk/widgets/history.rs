use super::history_entry::HistoryLineEntry;
use relm4::{
    factory::FactoryVecDeque,
    gtk,
    ComponentSender,
    ComponentParts,
    Component,
};

pub type HistoryEntries = FactoryVecDeque<HistoryLineEntry>;

#[derive(Debug)]
pub struct HistoryViewModel {
    pub history: HistoryEntries,
}

// Newtype wrapper for GtkScrolledWindow 
#[derive(Debug)]
pub struct HistoryPanel(gtk::ScrolledWindow);

#[derive(Debug)]
pub struct HistoryViewWidgets {
    pub history_panel: HistoryPanel,
}

pub trait HistoryPanelActions {
    fn inner(&self) -> &gtk::ScrolledWindow;
}

impl HistoryPanelActions for HistoryPanel {
    fn inner(&self) -> &gtk::ScrolledWindow { self.0.as_ref() }
}

// Test out adding a few entries
fn populate_history(mut history: HistoryEntries) {
    let clipboard = vec![
        "This".to_owned(),
        "Will".to_owned(),
        "Be".to_owned(),
        "Copied".to_owned(),
    ];

    clipboard.into_iter().for_each(|line| {
        /*
        let index = history.guard().push_back(line);
        history.guard().move_front(index.current_index());
        */
        let index = history.guard().push_back(line);
    });
}

impl Component for HistoryViewModel {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = gtk::ScrolledWindow;
    type Widgets = HistoryViewWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        gtk::ScrolledWindow::new()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        // Create model
        let mut history = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());
        let model = HistoryViewModel { history };

        // Set CSS
        // TODO: Create separate css file with global styles
        relm4::set_global_css_from_file("src/gtk/widgets/history.css");
        populate_history(history);

        // Create widgets
        let history_window = HistoryPanel(root.to_owned());
        let widgets = HistoryViewWidgets { history_panel: history_window };

        // Display the widgets
        let history_box = model.history.widget();
        widgets.history_panel.inner().set_child(Some(history_box));

        ComponentParts { model, widgets }
    }

    /*
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            HistoryInput::DeleteEntry(index) => {
                println!("HistoryInput::DeleteEntry");
                //let last = self.history.len() - 1;
                //self.history.guard().move_back(index.current_index());
                //self.history.guard().move_to(index.current_index(), last);

                //self.history.guard().move_to(index.current_index(), 0);
                //self.history.guard().remove(last);
                self.history.broadcast(HistoryEntryInput::RefreshIndex);
                //self.history.broadcast(HistoryEntryInput::DecrementIndex);
                //self.history.guard().remove(0);
                self.history.guard().remove(index.current_index());
                //self.history.guard().remove(last);

                /*
                self.history.guard().remove(index.current_index());

                // Ensure all the above entries are not affected
                let last = self.history.len();
                */
                //for i in index.current_index() - 1..last {
                //for i in index.current_index()..last {

                /*
                for i in index.current_index() - 1..last - 1 {
                //for i in index.current_index() - 1..last - 2 {
                    //if let Some(child) = self.history.get(i) {
                    if let Some(child) = self.history.get(i) {
                        //let new_index = DynamicIndex::from(i - 1);
                        //child.update(HistoryEntryInput::UpdateIndex(new_index), sender)
                        //child.update(HistoryEntryInput::UpdateIndex(i - 1), sender)
                        //child.update(HistoryEntryInput::DecrementIndex, sender);

                        //self.history.send(i, HistoryEntryInput::DecrementIndex);
                        self.history.send(i, HistoryEntryInput::RefreshIndex);
                    }
                }
            }
                */
            }
            HistoryInput::CopyEntry(index) => {
                // Copy the thing
            }
        }
    }
*/
}
