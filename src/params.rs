// use adw::prelude::BinExt;

use gtk::{
    prelude::{BoxExt, ButtonExt, OrientableExt, ListModelExt, EditableExt, StaticType, Cast, ObjectExt, WidgetExt},
    gio,
    };

use relm4::{
    gtk, send,
    MicroComponent, MicroModel, MicroWidgets, ComponentUpdate, Model, Sender, Widgets,
    factory::{Factory, FactoryVecDeque, FactoryPrototype, DynamicIndex, WeakDynamicIndex},
};

use crate::{AppModel, AppMsg};
use crate::sim::{Radical, Nucleus, Param};
use crate::nuc_object::NucObject;

// NucPar Component
// TODO: use the GLib Object

#[derive(Debug)]
enum NucParMsg {
    Add(String),
    RemoveLast,
}

#[derive(Debug)]
struct NucParModel {
    eqs: usize,
    spin_val: f64,
    spin_var: f64,
    hpf_val: f64,
    hpf_var: f64,
}

impl NucParModel {
    fn new() -> Self {
        NucParModel {
            eqs: 1,
            spin_val: 0.0,
            spin_var: 50.0,
            hpf_val: 0.0,
            hpf_var: 100.0,
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

    fn to_nuc(&self) -> Nucleus {
        Nucleus {
            eqs: Param::set(self.eqs as f64, 0.0),
            spin: Param::set(self.spin_val, self.spin_var),
            hpf: Param::set(self.hpf_val, self.hpf_var),
        }
    }
}

impl MicroModel for NucParModel {
    type Msg = NucParMsg;
    type Widgets = NucParWidgets;
    type Data = ();

   fn update(
        &mut self,
        msg: NucParMsg,
        _data: &(),
        sender: Sender<NucParMsg>,
    ) {
        match msg {
            _add => {}
            _remove => {}
        }
    }  // update
}

#[relm4::micro_widget]
#[derive(Debug)]
impl MicroWidgets<NucParModel> for NucParWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            append = &gtk::Label {
                set_label: watch!(&model.eqs.to_string()),
            },
            append = &gtk::Button {
                set_label: "Add",
                // connect_clicked(sender) => move |_| send!(sender, NucParMsg::Add),
            },
            append = &gtk::Button {
                set_label: "Del",
                // connect_clicked(sender) => move |_| send!(sender, NucParMsg::Remove),
            },
            append: eqs_entry = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                append: &gtk::Label::new(Some("Eqs")),
                append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
            },
            append: spin_entry = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                append: &gtk::Label::new(Some("Spin")),
                append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
            },
            append: hpf_entry = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                append: &gtk::Label::new(Some("Hpf")),
                append: &gtk::SpinButton::with_range(0.0, 100.0, 10.0),
            }
        }
    }
}  // impl for NucParWidgets

// Factory Microcomponent (manual)
// This is needed ONLY because I have to write widgets manually
// (without the `view` macro)
// So i prefer doing this in a compartimentalized microcomponent
// And not messing with the proper Relm structure of the remaining panel

struct NucFactoryModel {
    // TODO implement GLib Object subclass, then go back here
    // listbox_model: ListStore<NucParObject>,
    store: gio::ListStore,
}

impl NucFactoryModel {
    fn new() -> Self {
        NucFactoryModel {
            // listbox_model: ListStore::new(NucParObject::static_type()),
            store: gio::ListStore::new(NucObject::static_type()),
        }
    }
}

impl MicroModel for NucFactoryModel {
    type Msg = NucParMsg;
    type Widgets = NucFactoryWidgets;
    type Data = ();

   fn update(&mut self, msg: NucParMsg, _data: &(), _sender: Sender<NucParMsg>,) {
       match msg {
           NucParMsg::Add(_text) => {
               self.store.append(&NucObject::new());
           }
           NucParMsg::RemoveLast => {
               let index = self.store.n_items();
               if index != 0 {
                   self.store.remove(index - 1);
               }
           }
       }
    }  // update
}

#[derive(Debug)]
struct NucFactoryWidgets {
    main_box: gtk::Box,
}

impl MicroWidgets<NucFactoryModel> for NucFactoryWidgets {
    type Root = gtk::Box;

    fn init_view(model: &NucFactoryModel, sender: Sender<NucParMsg>) -> Self {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let name = gtk::Entry::builder().placeholder_text("Nucleus").build();
        let add = gtk::Button::with_label("Add new");
        let remove = gtk::Button::with_label("Remove selected");

        let scroller = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();

        main_box.append(&name);
        main_box.append(&add);
        main_box.append(&remove);
        main_box.append(&scroller);

        let sender2 = sender.clone();
        add.connect_clicked(move |_| {
            let text: String = name.text().into();
            send!(sender2, NucParMsg::Add(text));
        });

        remove.connect_clicked(move |_| {
            send!(sender, NucParMsg::RemoveLast);
        });

        let list_box = gtk::ListBox::new();

        list_box.bind_model(
            Some(&model.store),
            |item| {
                let eqs: i32 = item.property("eqs");
                let my_test_label = gtk::Label::new(Some(&eqs.to_string()));
                let result = my_test_label.ancestor(gtk::Widget::static_type());
                result.unwrap()
            }
        );

        scroller.set_child(Some(&list_box));

        NucFactoryWidgets { main_box }
    }

    fn view(&mut self, _model: &NucFactoryModel, _sender: Sender<NucParMsg>) {
        // Do things
    }

    fn root_widget(&self) -> Self::Root {
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
    nuc_factory: MicroComponent<NucFactoryModel>,
    nucs: Vec<MicroComponent<NucParModel>>,
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
            nuc_factory: MicroComponent::new(NucFactoryModel::new(), ()),
            nucs: Vec::new(),
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

        let nucs = self.nucs.iter().map(|nuc_component| {
            match nuc_component.model() {
                Ok(nuc_model) => {
                   nuc_model.to_nuc()
                }
                _ => {
                    // TODO raise error about nucleus conversion
                    Nucleus::set(0.0, 0.0, 0.0)
                }
            }
        }).collect::<Vec<Nucleus>>();


        Radical {
            lwa: Param::set(self.lwa_val, self.lwa_var),
            lrtz: Param::set(self.lrtz_val, self.lrtz_var),
            amount: Param::set(self.amount_val, self.amount_var),
            dh1: Param::set(self.dh1_val, self.dh1_var),
            nucs,
        }
    }
}

#[derive(Debug)]
pub enum RadParMsg {
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
    AddNuc(WeakDynamicIndex),
    RemoveNuc(WeakDynamicIndex),
}

pub struct RadParModel {
    pars: FactoryVecDeque<RadPar>,
    nuc_counter: u8,
    received_messages: u8,
}

// #[derive(relm4::Components)]
// pub struct RadParComponents {
//    nuc_factory: RelmComponent<NucParModel, RadParModel>,
// }

impl Model for RadParModel {
    type Msg = RadParMsg;
    type Widgets = RadParWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for RadParModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        RadParModel {
            pars: FactoryVecDeque::new(),
            nuc_counter: 0,
            received_messages: 0,
        }
    }

    fn update(
        &mut self,
        msg: RadParMsg,
        _components: &(),
        _sender: Sender<RadParMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            RadParMsg::AddFirst => {
                self.pars.push_front(RadPar::new(self.received_messages));
            }
            RadParMsg::RemoveLast => {
                self.pars.pop_back();
            }
            RadParMsg::Update => {
                // UpdateMain
                println!("Update Pars in main model");
                let mut new_rads: Vec<Radical> = Vec::new();
                for rad_par in self.pars.iter() {
                    new_rads.push(rad_par.to_rad());
                }

                send!(parent_sender, AppMsg::UpdateRads(new_rads));
            }
            RadParMsg::CountAt(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.value = counter.value.wrapping_sub(1);
                    }
                }
            }
            RadParMsg::RemoveAt(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.remove(index.current_index());
                }
            }
            RadParMsg::InsertBefore(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.insert(
                        index.current_index(),
                        RadPar::new(self.received_messages),
                    );
                }
            }
            RadParMsg::InsertAfter(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.insert(
                        index.current_index() + 1,
                        RadPar::new(self.received_messages),
                    );
                }
            }
            RadParMsg::SetLwaVal(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lwa_val = val;
                    }
                }
            }
            RadParMsg::SetLwaVar(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lwa_var = val;
                    }
                }
            }
            RadParMsg::SetLrtzVal(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lrtz_val = val;
                    }
                }
            }
            RadParMsg::SetLrtzVar(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.lrtz_var = val;
                    }
                }
            }
            RadParMsg::SetAmountVal(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.amount_val = val;
                    }
                }
            }
            RadParMsg::SetAmountVar(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.amount_var = val;
                    }
                }
            }
            RadParMsg::SetDh1Val(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.dh1_val = val;
                    }
                }
            }
            RadParMsg::SetDh1Var(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        counter.dh1_var = val;
                    }
                }
            }
            RadParMsg::AddNuc(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        // println!("Add Nuc to Radical Model with {} index", index.current_index());
                        counter.nucs.push(MicroComponent::new(NucParModel::new(), ()));
                        // Update counter
                        self.nuc_counter = self.nuc_counter.wrapping_add(1);
                    }
                }
            }
            RadParMsg::RemoveNuc(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        // println!("Remove last Nuc from Radical Model with {} index", index.current_index());
                        counter.nucs.pop();
                        // Update counter
                        self.nuc_counter = self.nuc_counter.wrapping_sub(1);
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
    type Msg = RadParMsg;

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
                            send!(sender, RadParMsg::CountAt(key.downgrade()));
                        }
                    },
                    append: remove_button = &gtk::Button {
                        set_label: "Remove",
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, RadParMsg::RemoveAt(key.downgrade()));
                        }
                    },
                    append: ins_above_button = &gtk::Button {
                        set_label: "Add above",
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, RadParMsg::InsertBefore(key.downgrade()));
                        }
                    },
                    append: ins_below_button = &gtk::Button {
                        set_label: "Add below",
                        connect_clicked(sender, key) => move |_| {
                            send!(sender, RadParMsg::InsertAfter(key.downgrade()));
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
                                send!(sender, RadParMsg::SetLwaVal(key.downgrade(), val.value()));
                            }
                        },
                        append: lwa_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, RadParMsg::SetLwaVar(key.downgrade(), val.value()));
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
                                send!(sender, RadParMsg::SetLrtzVal(key.downgrade(), val.value()));
                            }
                        },
                        append: lrtz_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, RadParMsg::SetLrtzVar(key.downgrade(), val.value()));
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
                                send!(sender, RadParMsg::SetAmountVal(key.downgrade(), val.value()));
                            }
                        },
                        append: amount_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, RadParMsg::SetAmountVar(key.downgrade(), val.value()));
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
                                send!(sender, RadParMsg::SetDh1Val(key.downgrade(), val.value()));
                            }
                        },
                        append: dh1_entry_var = &gtk::SpinButton {
                            set_adjustment: &RadPar::default_adjustment(),
                            connect_value_changed(sender, key) => move |val| {
                                send!(sender, RadParMsg::SetDh1Var(key.downgrade(), val.value()));
                            }
                        },
                    },
                },
                append: nuc_factory_box = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 10,
                        append = &gtk::Button {
                            set_label: "Add Nuc",
                            connect_clicked(sender, key) => move |val| {
                                send!(sender, RadParMsg::AddNuc(key.downgrade()));
                            }
                        },
                        append = &gtk::Button::with_label("Remove Nuc") {
                            connect_clicked(sender, key) => move |val| {
                                send!(sender, RadParMsg::RemoveNuc(key.downgrade()));
                            }
                        },
                    },
                    append: nucs_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 10,
                        // TODO del this one...
                        append: nucs_listbox = &gtk::ListBox {},
                        // for this one... Maintaining both until I'm not 100% sure about this change
                        append: self.nuc_factory.root_widget(),
                    }
                }
            },
            append: &gtk::Separator::new(gtk::Orientation::Horizontal),
        }
    }

    fn pre_view() {
        widgets.nucs_listbox.append(self.nucs.last().unwrap().root_widget());
    }

    fn position(&self, _index: &DynamicIndex) {}
}

pub struct RadParWidgets {
    main_box: gtk::Box,
    gen_box: gtk::Box,
}

impl Widgets<RadParModel, AppModel> for RadParWidgets {
    type Root = gtk::Box;

    fn init_view(_model: &RadParModel, _components: &(), sender: Sender<RadParMsg>) -> Self {
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

        let add = gtk::Button::with_label("Add Rad");
        let remove = gtk::Button::with_label("Remove Rad");
        let update = gtk::Button::with_label("Update Simulator");
        // TODO cancel function
        // let cancel = gtk::Button::with_label("Cancel");

        main_box.append(&gen_box);

        main_box.append(&add);
        main_box.append(&remove);
        main_box.append(&update);

        let sender_cloned_0 = sender.clone();
        let sender_cloned_1 = sender.clone();

        add.connect_clicked(move |_| {
            sender_cloned_0.send(RadParMsg::AddFirst).unwrap();
        });

        remove.connect_clicked(move |_| {
            sender_cloned_1.send(RadParMsg::RemoveLast).unwrap();
        });

        update.connect_clicked(move |_| {
            sender.send(RadParMsg::Update).unwrap();
        });

        RadParWidgets { main_box, gen_box }
    }

    fn view(&mut self, model: &RadParModel, sender: Sender<RadParMsg>) {
        model.pars.generate(&self.gen_box, sender);
    }

    fn root_widget(&self) -> gtk::Box {
        self.main_box.clone()
    }
}
