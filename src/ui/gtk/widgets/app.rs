use crate::{ui::gtk::widgets::{
    history_view::HistoryViewModel,
    configure_dialog::{ConfigureDialog, ConfigureDialogInput},
    device_panel::DevicePanelModel,
}, connect::client::ClientPool};

use gtk::prelude::{
    BoxExt,
    ButtonExt,
    GtkWindowExt,
    OrientableExt,
    WidgetExt,
};

use relm4::{
    component,
    Component,
    ComponentController,
    ComponentParts,
    ComponentSender,
    Controller,
    gtk,
    RelmWidgetExt,
    SimpleComponent,
};

pub struct App {
    clients: ClientPool,
    device_dialog: Controller<ConfigureDialog>,
    device_panel: Controller<DevicePanelModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    ShowDeviceConfigureDialog,
    AddDevice,
    RemoveDevice,
    //TODO
}

pub struct AppInit {}

#[component(pub)]
impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
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
                    set_spacing: 5,

                    gtk::Button {
                        set_label: "Settings",
                    },
                    gtk::Button {
                        set_label: "Shortcuts",
                    },
                    gtk::Button {
                        set_label: "Configure",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::ShowDeviceConfigureDialog);
                        }
                    },
                },
                #[name="mainview"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_all: 5,
                    set_hexpand: true,
                    set_vexpand: true,

                    #[local_ref]
                    device_panel_widget -> gtk::Notebook {
                    }
                    //device_view_widget -> gtk::Notebook {
                    //}

                    /*
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

                        // Example Device 1
                        append_page[Some(&gtk::Label::new(Some("Example Device 1")))] = &gtk::ScrolledWindow {
                            set_size_request: (300, 300),
                            set_hexpand: true,
                            set_vexpand: true,
                            gtk::Label {
                                set_label: "Example Device 1",
                            },
                            #[local_ref]
                            history_widget -> gtk::ScrolledWindow {
                            },
                        },
                    },
                    */
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        // Load all the clients up on startup
        let clients = ClientPool::new();
        //let clients = tokio::task::spawn_blocking(async move { clients.populate() });
        let clients = futures::executor::block_on(clients.populate());




        //let history = HistoryViewModel::builder()
            //.launch(()).detach();
            //.launch(()).forward(sender.input_sender(), |msg| ());

        let device_dialog = ConfigureDialog::builder()
            .launch(true)
            .detach();
            //.forward(sender.input_sender(), |msg| match msg {
                ////DialogOutput::Close => AppMsg::Close,
            //});

        //let device_view = DeviceView::builder().launch(()).detach();
        //let model = App { clients, history, device_dialog, device_view };

        let device_panel = DevicePanelModel::builder().launch(clients.clone()).detach();

        let model = App { clients, device_dialog, device_panel };

        //let history_widget = model.history.widget();
        model.device_dialog.widget().set_transient_for(Some(root));
        //let device_view_widget = model.device_view.widget();
        let device_panel_widget = model.device_panel.widget();

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::ShowDeviceConfigureDialog => {
                self.device_dialog.sender().send(ConfigureDialogInput::Show).unwrap();
            },
            AppMsg::AddDevice => {
            },
            AppMsg::RemoveDevice => {
            },
        }
    }
}
