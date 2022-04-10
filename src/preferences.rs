use crate::{adw, gtk, send, AppMsg, AppModel, Sender, Widgets, ComponentUpdate, Model};

use gtk::prelude::{WidgetExt, GtkWindowExt};

use adw::{
    prelude::{PreferencesWindowExt, AdwWindowExt, PreferencesPageExt, PreferencesGroupExt, PreferencesRowExt},
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
            // Already made manually: could replace with this
            // set_hide_on_close: true,
            set_modal: true,

            // set_content = Some(&adw::PreferencesGroup) {            } // Group

            add = &adw::PreferencesPage {
                set_name: Some("Nome"),
                set_title: "Titolo",

                add = &adw::PreferencesGroup {
                    set_title: "Nome gruppo",
                    set_description: Some("Descrizione gruppo blabla"),
                    add= &adw::ActionRow {
                        set_title: "riga"
                    },
                    add = &adw::ActionRow {
                        set_title: "un'altra riga"
                    },
                },
            }

        }
    }  // view macro

    fn pre_view() {
        let sender1 = sender.clone();
        self.root_widget().connect_close_request(move |_| {
            // &model.is_active = false;
            send!(sender1, PreferencesMsg::Hide);
            gtk::Inhibit(true)
        });
    }
}
