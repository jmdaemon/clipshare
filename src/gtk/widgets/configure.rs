use gtk::prelude::{
    BoxExt,
    ButtonExt,
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
pub enum DeviceInput {
    FindDevices,
    PairDevice,
    UnpairDevice,
}

#[derive(Debug)]
pub enum DeviceOutput {}

#[relm4::component(pub)]
impl SimpleComponent for DevicesView {
    type Input = DeviceInput;
    type Output = DeviceOutput;
    type Init = ();

    // TODO: Implement Drop that shuts down the widget discover daemon
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 12,
            set_margin_end: 0,

            set_spacing: 12,
            set_width_request: 100,
            // Current Device Name
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Label {
                    set_height_request: 24,
                    set_width_chars: 32,
                    set_label: "Current Hostname",
                },
                gtk::Button {
                    set_label: "Edit",
                },
            },

            // Paired Devices
            gtk::ListBox {
                set_vexpand: true,

                gtk::Label {
                    set_height_request: 24,
                    set_label: "Arch Laptop",
                },
                gtk::Label {
                    set_height_request: 24,
                    set_label: "Arch Desktop",
                },
            },
            // Refresh Button
            gtk::Button {
                set_valign: gtk::Align::End,
                set_label: "Refresh",
            },
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
            DeviceInput::FindDevices => {
                // Start the daemon
            },
            DeviceInput::PairDevice => {
                // Create device config
            },
            DeviceInput::UnpairDevice => {
                // Remove device config
            },
        }
    }
}
