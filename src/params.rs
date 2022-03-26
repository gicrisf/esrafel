// use adw::prelude::BinExt;

use gtk::{
    prelude::{BoxExt, ButtonExt, OrientableExt},
    };

use relm4::{
    gtk, send,
    ComponentUpdate, Model, Sender, Widgets,
    factory::{Factory, FactoryVec, FactoryVecDeque, FactoryPrototype, DynamicIndex, WeakDynamicIndex},
};

use crate::{AppModel, AppMsg};
use crate::sim::{Radical, Param};

// NucPar Component

enum NucParMsg {
    Add,
    Remove,
}

struct NucPar {
    eqs: usize,
    spin: f64,
    hpf: f64,
}

struct NucParModel {
    nucs: FactoryVec<NucPar>,
    created_nucs: u8,
}

impl Model for NucParModel {
    type Msg = NucParMsg;
    type Widgets = NucParWidgets;
    type Components = ();
}

impl ComponentUpdate<RadParModel> for NucParModel {
    fn init_model(_parent_model: &RadParModel) -> Self {
        NucParModel {
            nucs: FactoryVec::new(),
            created_nucs: 0,
        }  // NucParModel
    }  // init_model

    fn update(
        &mut self,
        msg: NucParMsg,
        _components: &(),
        sender: Sender<NucParMsg>,
        parent_sender: Sender<ParMsg>
    ) {
        match msg {
            Add => {}
            Remove => {}
        }
    }  // update
}  // impl for NucParModel

#[derive(Debug)]
struct NucFactoryWidgets {
    nuc_box: gtk::Box,
    nuc_lbl: gtk::Label,
}

impl FactoryPrototype for NucPar {
    type Factory = FactoryVec<Self>;
    type Widgets = NucFactoryWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Msg = NucParMsg;

    fn init_view(&self, index: &usize, sender: Sender<NucParMsg>) -> NucFactoryWidgets {
        let nuc_box = gtk::Box::new(gtk::Orientation::Vertical, 15);
        let nuc_lbl = gtk::Label::new(Some("Nuc"));
        nuc_box.append(&nuc_lbl);

        NucFactoryWidgets { nuc_box, nuc_lbl }
    }

    fn position(&self, _index: &usize) {}

    fn view(&self, _index: &usize, widgets: &NucFactoryWidgets) {
        widgets.nuc_lbl.set_label(&self.spin.to_string());
    }

    fn root_widget(widgets: &NucFactoryWidgets) -> &gtk::Box {
        &widgets.nuc_box
    }
}

pub struct NucParWidgets {
    main_box: gtk::Box,
    gen_box: gtk::Box,
}

impl Widgets<NucParModel, AppModel> for NucParWidgets {
    type Root = gtk::Box;

    fn init_view(_model: &NucParModel, _components: &(), sender: Sender<NucParMsg>) -> Self {
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

        NucParWidgets { main_box, gen_box }
    }

    fn view(&mut self, model: &NucParModel, sender: Sender<NucParMsg>) {
        model.nucs.generate(&self.gen_box, sender);
    }

    fn root_widget(&self) -> gtk::Box {
        self.main_box.clone()
    }
}

// RadPar Factory

struct RadPar {
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
    nucs: FactoryVec<NucPar>,
}

impl RadPar {
    fn new(v: u8) -> Self {
        RadPar {
            value: v,
            lwa_val: 0.0,
            lwa_var: 0.0,
            lrtz_val: 50.0,
            lrtz_var: 0.0,
            amount_val: 100.0,
            amount_var: 0.0,
            dh1_val: 0.0,
            dh1_var: 0.0,
            nucs: FactoryVec::new(),
        }
    }

    fn default_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            0.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            10.0,  // step_increment
            100.0,  // page_increment
            1000.0  // page_size
        )
    }  // adjustment

    fn to_rad(&self) -> Radical {
        Radical {
            lwa: Param::set(self.lwa_val, self.lwa_var),
            lrtz: Param::set(self.lrtz_val, self.lrtz_var),
            amount: Param::set(self.amount_val, self.amount_var),
            dh1: Param::set(self.dh1_val, self.dh1_var),
            // TODO nucs setter
            nucs: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum ParMsg {
    AddFirst,
    RemoveLast,
    Update,
    CountAt(WeakDynamicIndex),
    RemoveAt(WeakDynamicIndex),
    InsertBefore(WeakDynamicIndex),
    InsertAfter(WeakDynamicIndex),
    SetLwaVal(WeakDynamicIndex, f64),
    SetLwaVar(WeakDynamicIndex, f64),
    SetLrtzVal(WeakDynamicIndex, f64),
    SetLrtzVar(WeakDynamicIndex, f64),
    SetAmountVal(WeakDynamicIndex, f64),
    SetAmountVar(WeakDynamicIndex, f64),
    SetDh1Val(WeakDynamicIndex, f64),
    SetDh1Var(WeakDynamicIndex, f64),
}

pub struct RadParModel {
    pars: FactoryVecDeque<RadPar>,
    received_messages: u8,
}

impl Model for RadParModel {
    type Msg = ParMsg;
    type Widgets = RadParWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for RadParModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        RadParModel {
            pars: FactoryVecDeque::new(),
            received_messages: 0,
        }
    }

    fn update(
        &mut self,
        msg: ParMsg,
        _components: &(),
        _sender: Sender<ParMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ParMsg::AddFirst => {
                self.pars.push_front(RadPar::new(self.received_messages));
            }
            ParMsg::RemoveLast => {
                self.pars.pop_back();
            }
            ParMsg::Update => {
                // UpdateMain
                println!("Update Pars in main model");
                let mut new_rads: Vec<Radical> = Vec::new();
                for rad_par in self.pars.iter() {
                    new_rads.push(rad_par.to_rad());
                }

                send!(parent_sender, AppMsg::UpdateRads(new_rads));
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
                        RadPar::new(self.received_messages),
                    );
                }
            }
            ParMsg::InsertAfter(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.insert(
                        index.current_index() + 1,
                        RadPar::new(self.received_messages),
                    );
                }
            }
            ParMsg::SetLwaVal(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lwa_val = val;
                    }
                }
            }
            ParMsg::SetLwaVar(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lwa_var = val;
                    }
                }
            }
            ParMsg::SetLrtzVal(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lrtz_val = val;
                    }
                }
            }
            ParMsg::SetLrtzVar(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lrtz_var = val;
                    }
                }
            }
            ParMsg::SetAmountVal(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.amount_val = val;
                    }
                }
            }
            ParMsg::SetAmountVar(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.amount_var = val;
                    }
                }
            }
            ParMsg::SetDh1Val(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.dh1_val = val;
                    }
                }
            }
            ParMsg::SetDh1Var(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.dh1_var = val;
                    }
                }
            }
        }
        self.received_messages += 1;
    }
}  // Component Update

#[relm4::factory_prototype]
impl FactoryPrototype for RadPar {
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
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetLwaVal(key.downgrade(), val.value()));
                            }
                        },
                        append: lwa_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetLwaVar(key.downgrade(), val.value()));
                            }
                        },
                    },
                    // LRTZ Box
                    append: lrtz_entry = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,
                        prepend: &gtk::Label::new(Some("Lrtz")),
                        append: lrtz_entry_val = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetLrtzVal(key.downgrade(), val.value()));
                            }
                        },
                        append: lrtz_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetLrtzVar(key.downgrade(), val.value()));
                            }
                        },
                    },
                    // Amount Box
                    append: amount_entry = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,
                        prepend: &gtk::Label::new(Some("Amount")),
                        append: amount_entry_val = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetAmountVal(key.downgrade(), val.value()));
                            }
                        },
                        append: amount_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetAmountVar(key.downgrade(), val.value()));
                            }
                        },
                    },
                    append: dh1_entry = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_homogeneous: true,
                        prepend: &gtk::Label::new(Some("dh1")),
                        append: dh1_entry_val = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetDh1Val(key.downgrade(), val.value()));
                            }
                        },
                        append: dh1_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, ParMsg::SetDh1Var(key.downgrade(), val.value()));
                            }
                        },
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

pub struct RadParWidgets {
    main_box: gtk::Box,
    gen_box: gtk::Box,
}

impl Widgets<RadParModel, AppModel> for RadParWidgets {
    type Root = gtk::Box;

    fn init_view(_model: &RadParModel, _components: &(), sender: Sender<ParMsg>) -> Self {
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
        let update = gtk::Button::with_label("Update");
        // TODO cancel function
        // let cancel = gtk::Button::with_label("Cancel");

        main_box.append(&gen_box);

        main_box.append(&add);
        main_box.append(&remove);
        main_box.append(&update);

        let sender_cloned_0 = sender.clone();
        let sender_cloned_1 = sender.clone();

        add.connect_clicked(move |_| {
            sender_cloned_0.send(ParMsg::AddFirst).unwrap();
        });

        remove.connect_clicked(move |_| {
            sender_cloned_1.send(ParMsg::RemoveLast).unwrap();
        });

        update.connect_clicked(move |_| {
            sender.send(ParMsg::Update).unwrap();
        });

        RadParWidgets { main_box, gen_box }
    }

    fn view(&mut self, model: &RadParModel, sender: Sender<ParMsg>) {
        model.pars.generate(&self.gen_box, sender);
    }

    fn root_widget(&self) -> gtk::Box {
        self.main_box.clone()
    }
}
