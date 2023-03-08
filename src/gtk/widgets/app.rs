use gtk::prelude::{
    ButtonExt,
    GtkWindowExt,
    OrientableExt,
    WidgetExt,
};

use relm4::{
    component,
    gtk,
    SimpleComponent,
    ComponentSender,
    ComponentParts,
    RelmWidgetExt,
};

#[derive(Debug)]
pub struct App {

}

#[derive(Debug)]
pub enum AppMsg {}

pub struct AppInit {}

#[component(pub)]
impl SimpleComponent for App {
    type Input = ();
    type Output = AppMsg;
    type Init = AppInit;

    view! {

        gtk::Window {
            set_title: Some("Clipshare"),
            set_default_width: 640,
            set_default_height: 480,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                #[name="toolbar"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_all: 5,

                    gtk::Button {
                        set_label: "Settings",
                    },
                    gtk::Button {
                        set_label: "Shortcuts",
                    },
                },
                #[name="mainview"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_all: 5,
                    set_hexpand: true,
                    set_vexpand: true,

                    gtk::Notebook {
                        set_hexpand: true,
                        set_vexpand: true,

                        // Home View
                        append_page[Some(&gtk::Label::new(Some("Main")))] = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_margin_all: 5,
                            set_size_request: (100, 100),
                            set_hexpand: true,
                            set_vexpand: true,


                            gtk::Label {
                                set_label: "Main"
                            },
                        },
                        //} -> {
                            //set_tab_label: "Main",
                            //set_title: "Main",
                            //set_name: "Main",
                        //},

                        // Example Device 1
                        append_page[Some(&gtk::Label::new(Some("Example Device 1")))] = &gtk::ScrolledWindow {
                            set_size_request: (300, 300),
                            set_hexpand: true,
                            set_vexpand: true,
                            gtk::Label {
                                set_label: "Example Device 1",
                            },
                        },
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let model = App {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {
        //match message {
            //_ => todo!()
        //}
    }
}
