// use adw::prelude::BinExt;

use gtk::{
    prelude::{BoxExt, ButtonExt, OrientableExt},
    Orientation,
    };

use relm4::{
    gtk, send,
    ComponentUpdate, Model, Sender, Widgets,
    factory::{Factory, FactoryVecDeque, FactoryPrototype, DynamicIndex, WeakDynamicIndex},
};

use crate::{AppModel, AppMsg};
// use crate::sim::Radical;

struct Par {
    value: u8,
    lwa_val: f64,
    lwa_var: f64,
    // lwb_val: f64,
    // lwb_var: f64,
    // lwc_val: f64,
    // lwc_var: f64,
    lrtz_val: f64,
    lrtz_var: f64,
    amount_val: f64,
    amount_var: f64,
    dh1_val: f64,
    dh1_var: f64,
}

#[derive(Debug)]
pub enum ParMsg {
    AddFirst,
    RemoveLast,
    CountAt(WeakDynamicIndex),
    RemoveAt(WeakDynamicIndex),
    InsertBefore(WeakDynamicIndex),
    InsertAfter(WeakDynamicIndex),
    SetLwaVal(WeakDynamicIndex, f64),
    SetLwaVar(WeakDynamicIndex, f64),
    // SetLrtzVal(WeakDynamicIndex, f64),
    // SetLrtzVar(WeakDynamicIndex, f64),
    // SetAmountVal(WeakDynamicIndex, f64),
    // SetAmountVar(WeakDynamicIndex, f64),
    // SetDh1Val(WeakDynamicIndex, f64),
    // SetDh1Var(WeakDynamicIndex, f64),
}

pub struct ParamsModel {
    pars: FactoryVecDeque<Par>,
    received_messages: u8,
}

impl Model for ParamsModel {
    type Msg = ParMsg;
    type Widgets = ParamsWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ParamsModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ParamsModel {
            pars: FactoryVecDeque::new(),
            received_messages: 0,
        }
    }

    fn update(
        &mut self,
        msg: ParMsg,
        _components: &(),
        _sender: Sender<ParMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ParMsg::AddFirst => {
                self.pars.push_front(Par {
                    value: self.received_messages,
                    lwa_val: 0.0,
                    lwa_var: 0.0,
                    lrtz_val: 50.0,
                    lrtz_var: 0.0,
                    amount_val: 100.0,
                    amount_var: 0.0,
                    dh1_val: 0.0,
                    dh1_var: 0.0,
                });
            }
            ParMsg::RemoveLast => {
                self.pars.pop_back();
            }
            ParMsg::CountAt(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.value = counter.value.wrapping_sub(1);
                    }
                }
            }
            ParMsg::RemoveAt(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.remove(index.current_index());
                }
            }
            ParMsg::InsertBefore(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.insert(
                        index.current_index(),
                        Par {
                            value: self.received_messages,
                            lwa_val: 0.0,
                            lwa_var: 0.0,
                            lrtz_val: 50.0,
                            lrtz_var: 0.0,
                            amount_val: 100.0,
                            amount_var: 0.0,
                            dh1_val: 0.0,
                            dh1_var: 0.0,
                        },
                    );
                }
            }
            ParMsg::InsertAfter(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.insert(
                        index.current_index() + 1,
                        Par {
                            value: self.received_messages,
                            lwa_val: 0.0,
                            lwa_var: 0.0,
                            lrtz_val: 50.0,
                            lrtz_var: 0.0,
                            amount_val: 100.0,
                            amount_var: 0.0,
                            dh1_val: 0.0,
                            dh1_var: 0.0,
                        },
                    );
                }
            }
            ParMsg::SetLwaVal(weak_index, val) => {
                println!("New Lwa Val for Radical {:?}: {}", weak_index.upgrade(), val);
            }
            ParMsg::SetLwaVar(weak_index, val) => {
                println!("New Lwa Var for Radical {:?}: {}", weak_index.upgrade(), val);
            }
        }
        self.received_messages += 1;
    }
}  // Component Update

#[relm4::factory_prototype]
impl FactoryPrototype for Par {
    type Factory = FactoryVecDeque<Self>;
    type Widgets = FactoryWidgets;
    type View = gtk::Box;
    type Msg = ParMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 15,
            append: label_box = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                append: &gtk::Label::new(Some("Radical")),
            },
            append: entries_box = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 15,
                append: par_general_box = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    append: counter_button = &gtk::Button {
                        set_label: watch!(&self.value.to_string()),
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, ParMsg::CountAt(key.downgrade()));
                        }
                    },
                    append: remove_button = &gtk::Button {
                        set_label: "Remove",
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, ParMsg::RemoveAt(key.downgrade()));
                        }
                    },
                    append: ins_above_button = &gtk::Button {
                        set_label: "Add above",
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, ParMsg::InsertBefore(key.downgrade()));
                        }
                    },
                    append: ins_below_button = &gtk::Button {
                        set_label: "Add below",
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, ParMsg::InsertAfter(key.downgrade()));
                        }
                    },
                },
                append: rad_params_box = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_homogeneous: true,

                    // LWA Box
                    append: lwa_entry_old = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,

                        prepend: &gtk::Label::new(Some("LWA")),

                        append: lwa_entry_val = &gtk::SpinButton {
                            set_adjustment: &gtk::Adjustment::new(
                                0.0,  // value
                                0.0,  // lower
                                100000000.0,  // upper
                                10.0,  // step_increment
                                100.0,  // page_increment
                                1000.0  // page_size
                            ),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetLwaVal(key.downgrade(), val.value()));
                            }
                        },

                        append: lwa_entry_var = &gtk::SpinButton {
                            set_adjustment: &gtk::Adjustment::new(
                                0.0,  // value
                                0.0,  // lower
                                100000000.0,  // upper
                                10.0,  // step_increment
                                100.0,  // page_increment
                                1000.0  // page_size
                            ),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetLwaVar(key.downgrade(), val.value()));
                            }
                        },

                    },
                    append: lrtz_entry = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,
                        append: &gtk::Label::new(Some("Lrtz")),
                        append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                        append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                    },
                    append: amount_entry = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,
                        append: &gtk::Label::new(Some("Amount")),
                        append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                        append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                    },
                    append: dh1_entry = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,
                        append: &gtk::Label::new(Some("dh1")),
                        append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                        append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                    },
                },
                append: nucs_box = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    append: nuc_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        append: eqs_entry = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 10,
                            append: &gtk::Label::new(Some("Eqs")),
                            append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                        },
                        append: spin_entry = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 10,
                            // Could justify the text, but not gonna do this in a stage this early
                            append: &gtk::Label::new(Some("Spin")),
                            append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                            append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                        },
                        append: hpf_entry = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 10,
                            append: &gtk::Label::new(Some("Hpf")),
                            append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                            append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
                        },
                    }
                }
            },
            append: &gtk::Separator::new(gtk::Orientation::Horizontal),
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

    fn init_view(_model: &ParamsModel, _components: &(), sender: Sender<ParMsg>) -> Self {
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

        main_box.append(&gen_box);

        main_box.append(&add);
        main_box.append(&remove);
        // main.set_child(Some(&main_box));

        let cloned_sender = sender.clone();
        add.connect_clicked(move |_| {
            cloned_sender.send(ParMsg::AddFirst).unwrap();
        });

        remove.connect_clicked(move |_| {
            sender.send(ParMsg::RemoveLast).unwrap();
        });

        ParamsWidgets { main_box, gen_box }
    }

    fn view(&mut self, model: &ParamsModel, sender: Sender<ParMsg>) {
        model.pars.generate(&self.gen_box, sender);
    }

    fn root_widget(&self) -> gtk::Box {
        self.main_box.clone()
    }
}
