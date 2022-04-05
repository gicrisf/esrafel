use crate::{AppModel, AppMsg};
use crate::{adw, gtk};

use relm4::{Widgets, Sender, ComponentUpdate, Model};
use gtk::{
    prelude::{WidgetExt, OrientableExt}
};

// Status Component
//
// TODO logging in categories (error, warning, simple info updates...)
// Keeping this simple for the moment

pub struct StatusModel {
    log: Vec<String>,
    last_toast: Option<adw::Toast>,
}

pub enum StatusMsg {
    New(String),
}

impl Model for StatusModel {
    type Msg = StatusMsg;
    type Widgets = StatusWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for StatusModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        StatusModel {
            log: Vec::new(),
            last_toast: None,
        }
    }

    fn update (&mut self, msg: StatusMsg, _components: &(), _sender: Sender<StatusMsg>, _parent_sender: Sender<AppMsg>) {
        match msg {
            StatusMsg::New(msg) => {
                self.last_toast = Some(adw::Toast::new(&msg));
                self.log.push(msg);
            }
        }  // match
    }
}

#[relm4::widget(pub)]
impl Widgets<StatusModel, AppModel> for StatusWidgets {
    view! {
        adw::ToastOverlay::new() {
            set_child = Some(&gtk::Box) {
                set_margin_top: 85,
                set_orientation: gtk::Orientation::Horizontal,
            },
        }  // root widget
    }

    fn pre_view() {
        if let Some(toast) = &model.last_toast {
            self.root_widget().add_toast(&toast);
        }
    }
}
