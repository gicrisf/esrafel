use crate::{adw, gtk, send, AppMsg, AppModel, Sender, Widgets, ComponentUpdate, Model};

use gtk::prelude::{WidgetExt, GtkWindowExt};

pub struct AboutModel {
    is_active: bool,
    // authors: Vec<String>,
    // artists: Vec<String>,
    comments: String,  // TODO static &str
    // copyright: String,
    // documenters: Vec<String>,
    // license: String,
    // license_type: gtk::License,
    // logo: gtk::Paintable,
    // logo_icon_name: String,
    program_name: String,
    // system_information: String,
    // translator_credits: String,
    version: String,
    // website: String,
    // website_label: String,
    // wrap_license: book,
}

pub enum AboutMsg {
    Show,
    Hide,
}

impl Model for AboutModel {
    type Msg = AboutMsg;
    type Widgets = AboutWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for AboutModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        AboutModel {
            is_active: false,
            // authors: vec!["Giovanni Crisalfi".into()],
            comments: "Yeah".into(),
            program_name: "ESRafel".into(),
            version: "0.1.0".into(),
        }
    }

    fn update(
        &mut self,
        msg: AboutMsg,
        _components: &(),
        _sender: Sender<AboutMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            AboutMsg::Show => self.is_active = true,
            AboutMsg::Hide => self.is_active = false,
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<AboutModel, AppModel> for AboutWidgets {
    view! {
        win = gtk::AboutDialog {
            set_visible: watch!(model.is_active),
            set_modal: true,
            // set_authors: &model.authors,
            set_comments: Some(&model.comments),
            // set_copyright: Some(&model.copyright),
            set_program_name: Some(&model.program_name),
            // set_license: Some(&model.license),
            set_version: Some(&model.version),
        }
    }  // view macro

    fn pre_view() {
        let sender1 = sender.clone();
        self.root_widget().connect_close_request(move |_| {
            // &model.is_active = false;
            send!(sender1, AboutMsg::Hide);
            gtk::Inhibit(true)
        });
    }
}
