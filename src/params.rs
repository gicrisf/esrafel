// use adw::prelude::BinExt;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};

use relm4::{
    gtk, send,
    ComponentUpdate, Model, Sender, Widgets,
    factory::{Factory, FactoryVecDeque, FactoryPrototype, DynamicIndex, WeakDynamicIndex},
};

use crate::{AppModel, AppMsg};
use crate::sim::Radical;

struct Counter {
    value: u8,
    rad: Radical,
}

#[derive(Debug)]
pub enum ParsMsg {
    AddFirst,
    RemoveLast,
    CountAt(WeakDynamicIndex),
    RemoveAt(WeakDynamicIndex),
    InsertBefore(WeakDynamicIndex),
    InsertAfter(WeakDynamicIndex),
}

pub struct ParamsModel {
    counters: FactoryVecDeque<Counter>,
    received_messages: u8,
}

impl Model for ParamsModel {
    type Msg = ParsMsg;
    type Widgets = ParamsWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ParamsModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ParamsModel {
            counters: FactoryVecDeque::new(),
            received_messages: 0,
        }
    }

    fn update(
        &mut self,
        msg: ParsMsg,
        _components: &(),
        _sender: Sender<ParsMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ParsMsg::AddFirst => {
                self.counters.push_front(Counter {
                    value: self.received_messages,
                    rad: Radical::electron(),
                });
            }
            ParsMsg::RemoveLast => {
                self.counters.pop_back();
            }
            ParsMsg::CountAt(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.counters.get_mut(index.current_index()) {
                        counter.value = counter.value.wrapping_sub(1);
                    }
                }
            }
            ParsMsg::RemoveAt(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.counters.remove(index.current_index());
                }
            }
            ParsMsg::InsertBefore(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.counters.insert(
                        index.current_index(),
                        Counter {
                            value: self.received_messages,
                            rad: Radical::electron(),
                        },
                    );
                }
            }
            ParsMsg::InsertAfter(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.counters.insert(
                        index.current_index() + 1,
                        Counter {
                            value: self.received_messages,
                            rad: Radical::electron(),
                        },
                    );
                }
            }
        }
        self.received_messages += 1;
    }
}  // Component Update

#[relm4::factory_prototype]
impl FactoryPrototype for Counter {
    type Factory = FactoryVecDeque<Self>;
    type Widgets = FactoryWidgets;
    type View = gtk::Box;
    type Msg = ParsMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            append: counter_button = &gtk::Button {
                set_label: watch!(&self.value.to_string()),
                connect_clicked(sender, key) => move |_| {
                    send!(sender, ParsMsg::CountAt(key.downgrade()));
                }
            },
            append: remove_button = &gtk::Button {
                set_label: "Remove",
                connect_clicked(sender, key) => move |_| {
                    send!(sender, ParsMsg::RemoveAt(key.downgrade()));
                }
            },
            append: ins_above_button = &gtk::Button {
                set_label: "Add above",
                connect_clicked(sender, key) => move |_| {
                    send!(sender, ParsMsg::InsertBefore(key.downgrade()));
                }
            },
            append: ins_below_button = &gtk::Button {
                set_label: "Add below",
                connect_clicked(key) => move |_| {
                    send!(sender, ParsMsg::InsertAfter(key.downgrade()));
                }
            }
        }
    }

    fn position(&self, _index: &DynamicIndex) {}
}

pub struct ParamsWidgets {
    main_box: gtk::Box,
    gen_box: gtk::Box,
}

impl Widgets<ParamsModel, AppModel> for ParamsWidgets {
    type Root = gtk::Box;

    fn init_view(_model: &ParamsModel, _components: &(), sender: Sender<ParsMsg>) -> Self {
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .spacing(5)
            .build();

        let gen_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .spacing(5)
            .build();

        let add = gtk::Button::with_label("Add");
        let remove = gtk::Button::with_label("Remove");

        main_box.append(&add);
        main_box.append(&remove);
        main_box.append(&gen_box);

        // main.set_child(Some(&main_box));

        let cloned_sender = sender.clone();
        add.connect_clicked(move |_| {
            cloned_sender.send(ParsMsg::AddFirst).unwrap();
        });

        remove.connect_clicked(move |_| {
            sender.send(ParsMsg::RemoveLast).unwrap();
        });

        ParamsWidgets { main_box, gen_box }
    }

    fn view(&mut self, model: &ParamsModel, sender: Sender<ParsMsg>) {
        model.counters.generate(&self.gen_box, sender);
    }

    fn root_widget(&self) -> gtk::Box {
        self.main_box.clone()
    }
}
