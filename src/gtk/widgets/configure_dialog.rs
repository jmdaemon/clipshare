use gtk::prelude::{
    ButtonExt,
    DialogExt,
    GtkWindowExt,
    ToggleButtonExt,
    WidgetExt
};
use relm4::{
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts
};

#[derive(Debug)]
pub struct ConfigureDialog {
    hidden: bool,
}

#[derive(Debug)]
pub enum ConfigureDialogInput {
    Show,
    Close,
}

#[derive(Debug)]
pub enum ConfigureDialogOutput {
    Close
}

//pub struct ConfigureDialogInit {}

#[relm4::component(pub)]
impl SimpleComponent for ConfigureDialog {
    type Init = bool;
    type Input = ConfigureDialogInput;
    type Output = ConfigureDialogOutput;

    view! {
        gtk::MessageDialog {
            set_modal: false,
            #[watch]
            set_visible: !model.hidden,
            //set_text: Some("Do you want to close before saving?"),
            //set_secondary_text: Some("All unsaved changes will be lost"),
            //add_button: ("Cancel", gtk::ResponseType::Cancel),
            add_button: ("Close", gtk::ResponseType::Close),
            connect_response[sender] => move |_, resp| {
                sender.input(ConfigureDialogInput::Close);
            },
                //sender.input(if resp == gtk::ResponseType::Accept {
                    //DialogInput::Accept
                //} else {
                    //DialogInput::Cancel
                //})
            //}
        }
    }

    fn init(
        hidden: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ConfigureDialog { hidden };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ConfigureDialogInput::Show => self.hidden = false,
            ConfigureDialogInput::Close => self.hidden = true,
        }
    }
}
