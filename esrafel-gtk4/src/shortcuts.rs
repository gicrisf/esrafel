use crate::{gtk, send, AppMsg, AppModel, Sender, Widgets, ComponentUpdate, Model};

use gtk::prelude::{WidgetExt, GtkWindowExt};

pub struct ShortcutsModel {
    is_active: bool,
}

pub enum ShortcutsMsg {
    Show,
    Hide,
}

impl Model for ShortcutsModel {
    type Msg = ShortcutsMsg;
    type Widgets = ShortcutsWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ShortcutsModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ShortcutsModel {
            is_active: false,
        }
    }

    fn update(
        &mut self,
        msg: ShortcutsMsg,
        _components: &(),
        _sender: Sender<ShortcutsMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ShortcutsMsg::Show => self.is_active = true,
            ShortcutsMsg::Hide => self.is_active = false,
        }
    }
}

pub struct ShortcutsWidgets {
    win: gtk::ShortcutsWindow,
}

impl Widgets<ShortcutsModel, AppModel> for ShortcutsWidgets {
    type Root = gtk::ShortcutsWindow;

    fn init_view(model: &ShortcutsModel, _components: &(), sender: Sender<ShortcutsMsg>) -> Self {
        let win = gtk::ShortcutsWindow::builder().build();

        ShortcutsWidgets { win }
    }

    fn view(&mut self, model: &ShortcutsModel, sender: Sender<ShortcutsMsg>) {
        self.win.set_visible(model.is_active);

        let sender1 = sender.clone();
        self.win.connect_close_request(move |_| {
            send!(sender1, ShortcutsMsg::Hide);
            gtk::Inhibit(true)
        });
    }

    fn root_widget(&self) -> Self::Root {
        self.win.clone()
    }
}
