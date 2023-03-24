use super::configure::DevicesView;

use gtk::prelude::{
    BoxExt,
    OrientableExt,
    ButtonExt,
    DialogExt,
    GtkWindowExt,
    ToggleButtonExt,
    WidgetExt,
};
use relm4::{
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts,
    Controller,
    Component,
    ComponentController,
    RelmWidgetExt,
};

#[derive(Debug)]
pub struct ConfigureDialog {
    hidden: bool,
    devices_view: Controller<DevicesView>,
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
        gtk::Window {
            set_modal: false,
            #[watch]
            set_visible: !model.hidden,
            //set_text: Some("Do you want to close before saving?"),
            //set_secondary_text: Some("All unsaved changes will be lost"),
            //add_button: ("Cancel", gtk::ResponseType::Cancel),
            //#[wrap(Some)]
            //set_child: devices_view -> gtk::Box,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                //model.devices_view.widget(),
                //#[local_ref]
                //devices_view_box -> gtk::Box,
                #[local_ref]
                devices_view_box -> gtk::Box {
                    //set_size_request: (320, 320),
                    set_width_request: 100,
                    set_vexpand: true,
                    set_hexpand: false,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_hexpand: true,
                    set_vexpand: true,
                    //set_margin_all: 5,
                    set_margin_all: 12,
                    set_margin_start: 0,

                    //set_margin_top: 12,
                    //set_margin_bottom: 12,
                    //set_margin_end: 12,

                    gtk::Frame {
                        set_hexpand: true,
                        set_vexpand: true,

                        gtk::Stack {
                            set_hexpand: true,
                            set_vexpand: true,
                            //set_height_request: 600,
                            //set_width_request: 600,
                        },
                    },
                    
                    gtk::Button {
                        //set_hexpand: false,
                        set_valign: gtk::Align::End,
                        set_halign: gtk::Align::End,
                        set_label: "Close",
                        //connect_response[sender] => move |_, resp| {
                        connect_clicked[sender] => move |_| {
                            sender.input(ConfigureDialogInput::Close);
                        },
                    },
                }
            }

            //add_controller: &devices_view,
            //add_button: ("Close", gtk::ResponseType::Close),
            //connect_response[sender] => move |_, resp| {
                //sender.input(ConfigureDialogInput::Close);
            //},
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
        let devices_view = DevicesView::builder()
            .launch(()).detach();

        let model = ConfigureDialog { hidden, devices_view };
        let devices_view_box = model.devices_view.widget();

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
