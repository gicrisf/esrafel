use crate::{adw, gtk, send, AppMsg, AppModel, Sender, Widgets, ComponentUpdate, Model};

use gtk::prelude::{WidgetExt, GtkWindowExt};

use adw::{
    prelude::{PreferencesWindowExt, AdwWindowExt, PreferencesPageExt,
              PreferencesGroupExt, PreferencesRowExt, ActionRowExt, ListBoxRowExt},
};

pub struct PreferencesModel {
    test_pref: bool,
    is_active: bool,
}

pub enum PreferencesMsg {
    Test,
    Show,
    Hide,
}

impl Model for PreferencesModel {
    type Msg = PreferencesMsg;
    type Widgets = PreferencesWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for PreferencesModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        PreferencesModel {
            test_pref: true,
            is_active: false,
        }
    }

    fn update(
        &mut self,
        msg: PreferencesMsg,
        _components: &(),
        _sender: Sender<PreferencesMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            PreferencesMsg::Test => self.test_pref = !self.test_pref,
            PreferencesMsg::Show => self.is_active = true,
            PreferencesMsg::Hide => self.is_active = false,
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<PreferencesModel, AppModel> for PreferencesWidgets {
    view! {
        win = adw::PreferencesWindow {
            set_search_enabled: false,
            set_visible: watch!(model.is_active),
            set_modal: true,
            set_destroy_with_parent: true,
            // set_position: gtk::WindowPosition::CenterOnParent,

            // Already made manually
            // set_hide_on_close: true,

            set_modal: true,
            add = &adw::PreferencesPage {
                set_name: Some("Preferences Page"),
                set_title: "Preferences",

                add = &adw::PreferencesGroup {
                    set_title: "Plotting",
                    set_description: Some("Colors"),
                    add= &adw::ActionRow {
                        set_title: "Background",
                        // add_suffix: &gtk::ColorButton { ... it doesn't support set_valign ...},
                        add_suffix: &gtk::builders::ColorButtonBuilder::new()
                            .valign(gtk::Align::Center)
                            .build(),
                    },
                    add = &adw::ActionRow {
                        set_title: "Empirical",
                        set_valign: gtk::Align::Center,
                        add_suffix: &gtk::builders::ColorButtonBuilder::new()
                            .valign(gtk::Align::Center)
                            .build(),
                    },
                    add = &adw::ActionRow {
                        set_title: "Theoretical",
                        set_valign: gtk::Align::Center,
                        add_suffix: &gtk::builders::ColorButtonBuilder::new()
                            .valign(gtk::Align::Center)
                            .build(),
                    },
                },
            }
        }
    }  // view macro

    fn pre_view() {
        // Could replace with set_hide_on_close
        let sender1 = sender.clone();
        self.root_widget().connect_close_request(move |_| {
            // &model.is_active = false;
            send!(sender1, PreferencesMsg::Hide);
            gtk::Inhibit(true)
        });
    }
}
