use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt,
    WidgetExt,
};

use relm4::{
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts,
    RelmWidgetExt,
};

#[derive(Debug)]
pub struct DevicesView {}

#[derive(Debug)]
pub enum DeviceInput {}

#[derive(Debug)]
pub enum DeviceOutput {}

//pub struct DeviceInit {}

#[relm4::component(pub)]
impl SimpleComponent for DevicesView {
    type Input = DeviceInput;
    type Output = DeviceOutput;
    type Init = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            //set_margin_all: 12,
            //set_margin: 12,
            //set_margin_all: 5,
            set_margin_all: 12,
            set_margin_end: 0,

            set_spacing: 12,
            set_width_request: 100,
            gtk::Box {
                //set_width_request: 200,
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Label {
                    set_height_request: 24,
                    //set_width_chars: 64,
                    set_width_chars: 32,
                    set_label: "Current Hostname",
                },
                gtk::Button {
                    set_label: "Edit",
                },
            },

            gtk::ListBox {
            //gtk::Box {
                //set_spacing: 6,
                //set_orientation: gtk::Orientation::Vertical,
                //set_width_request: 200,
                set_vexpand: true,

                gtk::Label {
                    //set_height_request: 32,
                    set_height_request: 24,
                    set_label: "Arch Laptop",
                },
                gtk::Label {
                    //set_height_request: 32,
                    set_height_request: 24,
                    set_label: "Arch Desktop",
                },
            },
            gtk::Button {
                //set_halign: gtk::Align::End,
                set_valign: gtk::Align::End,
                //set_hexpand: true,
                set_label: "Refresh",
            },
            //gtk::Stack {
            //},
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DevicesView {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {

        }
    }
}
