use crate::{AppModel, AppMsg};

use relm4::{Widgets, Sender, ComponentUpdate, Model};

use crate::adw;

// EventLog Component
// TODO distinguish between categories (error, warning, simple info updates...)
// Keeping this simple for the moment

pub struct EventLogModel {
    log: Vec<String>,
    last_toast: Option<adw::Toast>,
}

pub enum EventLogMsg {
    New(String),
}

impl Model for EventLogModel {
    type Msg = EventLogMsg;
    type Widgets = EventLogWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for EventLogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        EventLogModel {
            log: Vec::new(),
            last_toast: None,
        }
    }

    fn update (&mut self, msg: EventLogMsg, _components: &(), _sender: Sender<EventLogMsg>, _parent_sender: Sender<AppMsg>) {
        match msg {
            EventLogMsg::New(msg) => {
                self.last_toast = Some(adw::Toast::new(&msg));
                self.log.push(msg);
            }
        }  // match
    }
}

#[relm4::widget(pub)]
impl Widgets<EventLogModel, AppModel> for EventLogWidgets {
    view! {
        adw::ToastOverlay::new() {
            // set_margin_top: 50,
        }  // root widget
    }

    fn pre_view() {
        if let Some(toast) = &model.last_toast {
            self.root_widget().add_toast(&toast);
        }
    }
}
