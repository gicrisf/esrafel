use crate::{adw, gtk, send, AppMsg, AppModel, Sender, Widgets, ComponentUpdate, Model};

use gtk::prelude::{WidgetExt, GtkWindowExt};

pub struct AboutModel {
    is_active: bool,
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
            // set_artists: &["Giovanni Crisalfi"],
            set_authors: &["Giovanni Crisalfi"],
            set_comments: Some("Software for least-squares fitting of ESR/EPR spectra with Monte Carlo methods."),
            set_copyright: Some("© 2022 Giovanni Crisalfi"),
            // set_documenters: &["Giovanni Crisalfi"],
            set_license: Some("GPL3"),
            set_license_type: gtk::License::Gpl30,
            // set_logo: ,
            // set_logo_icon_name: ,
            set_program_name: Some("ESRafel"),
            // set_system_information: ?,
            // set_translator_credits: Some("Giovanni Crisalfi"),
            set_version: Some("0.1.0"),
            set_website: Some("https://www.github.com/gicrisf/esrafel/"),
            set_website_label: "github.com/gicrisf/esrafel",
        }
    }  // view macro

    fn pre_view() {
        let sender1 = sender.clone();
        self.root_widget().connect_close_request(move |_| {
            send!(sender1, AboutMsg::Hide);
            gtk::Inhibit(true)
        });
    }
}
