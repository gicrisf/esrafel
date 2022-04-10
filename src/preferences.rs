use crate::{adw, gtk, send, AppMsg, AppModel, Sender, Widgets, ComponentUpdate, Model};

use gtk::prelude::{WidgetExt, GtkWindowExt};

use adw::{
    prelude::{PreferencesWindowExt},
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
            set_search_enabled: true,
            set_visible: watch!(model.is_active),
            set_modal: true,
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
