use std::ops::{Sub, SubAssign};

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
pub struct HistoryLineEntry {
    pub index: DynamicIndex,
    pub text: String,
}

//#[derive(Debug, Clone)]
//pub enum HistoryEntryInput {
    ////UpdateIndex(DynamicIndex),
    //DecrementIndex,
    //RefreshIndex,
//}

#[derive(Debug)]
pub enum HistoryLineEntryActions {
    Copy(DynamicIndex),
    Delete(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for HistoryLineEntry {
    type Init = String;
    type Input = HistoryLineEntryActions;
    type Output = ();
    type CommandOutput = ();
    type ParentInput = ();
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
                set_label: &self.text,
                set_width_chars: 128,
                set_xalign: 0.00,
                //connect_clicked[sender] => move |_| {
                //}
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
                    //sender.output(HistoryLineEntryActions::Copy(index.clone()))
                }
            },

            #[name(btn_delete)]
            gtk::Button {
                set_height_request: 24,
                set_label: "Delete",
                connect_clicked[sender, index] => move |_| {
                    //sender.output(HistoryLineEntryActions::Delete(index.clone()))
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
            text: init,
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

    //fn output_to_parent_input(output: Self::Output) -> Option<HistoryInput> {
        //Some(match output {
            //HistoryLineEntryActions::Delete(index) => HistoryInput::DeleteEntry(index),
            //HistoryLineEntryActions::Copy(index) => HistoryInput::CopyEntry(index),
        //})
    //}

    //fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        //match message {
            //HistoryEntryInput::DecrementIndex => {
                //if self.index.current_index() > 0 {
                    //self.index.current_index().sub_assign(1);
                //}
                ////self.index.current_index().sub(1);
            //},
            //HistoryEntryInput::RefreshIndex => {
                //self.index = self.index.to_owned();
            //},
            ////HistoryEntryInput::UpdateIndex(index) => {
                ////self.index = index;
            ////}
        //}
    //}
}
