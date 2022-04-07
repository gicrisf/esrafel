use adw::prelude::BinExt;

use gtk::{
    prelude::{BoxExt, FrameExt, ButtonExt, OrientableExt, ListModelExt, EditableExt, StaticType, ObjectExt, WidgetExt, GridExt},
    gio, glib,
    };

use relm4::{
    gtk, send, adw,
    MicroComponent, MicroModel, MicroWidgets, ComponentUpdate, Model, Sender, Widgets,
    factory::{Factory, FactoryVecDeque, FactoryPrototype, DynamicIndex, WeakDynamicIndex},
};

use crate::{AppModel, AppMsg};
use crate::sim::{Radical, Nucleus, Param};
use crate::nuc_object::NucObject;

// NucPar Component
#[derive(Debug)]
enum NucParMsg {
    Add(String),
    RemoveLast,
}

// Factory Microcomponent (manual)
// This is needed ONLY because I have to write widgets manually
// (without the `view` macro)
// So i prefer doing this in a compartimentalized microcomponent
// And not messing with the proper Relm structure of the remaining panel

struct NucFactoryModel {
    store: gio::ListStore,
}

impl NucFactoryModel {
    fn new() -> Self {
        NucFactoryModel {
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

impl NucFactoryModel {
    // Convert every NucObject in a Nucleus struct
    fn collect_nucs(&self) -> Vec<Nucleus> {
        let index = self.store.n_items();
        let mut nucs_vec = Vec::new();

        if index != 0 {
            for idx in 0..index {
                // UNDOCUMENTED METHOD!
                let mynuc = self.store.item(idx);
                match mynuc {
                    Some(obj) => {
                        let nuc = Self::to_nuc(&obj);
                        nucs_vec.push(nuc);
                    }
                    None => {
                        // TODO manage error
                        // Just sending something to the main window via sender
                        // then show the error with a proper widget
                        // send!(sender, AppMsg::SpawnToast("Cannot collect Nucs from GUI"));
                    }
                }
            }
        }

        nucs_vec
    }

    fn to_nuc(obj: &glib::Object) -> Nucleus {
        let eqs_val: f32 = obj.property("eqs");
        let spin_val: f32 = obj.property("spinval");
        let spin_var: f32 = obj.property("spinvar");
        let hpf_val: f32 = obj.property("hpfval");
        let hpf_var: f32 = obj.property("hpfvar");

        Nucleus {
            eqs: Param::set(eqs_val as f64, 0.0),
            spin: Param::set(spin_val as f64, spin_var as f64),
            hpf: Param::set(hpf_val as f64, hpf_var as f64),
        }
    }
}

#[derive(Debug)]
struct NucFactoryWidgets {
    main_box: gtk::Box,
}

impl NucFactoryWidgets {
    fn eqs_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            1.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            1.0,  // step_increment
            10.0,  // page_increment
            100.0  // page_size
        )
    }  // adjustment

    fn spin_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            0.5,  // value
            -100.0,  // lower
            100000000.0,  // upper
            0.5,  // step_increment
            1.0,  // page_increment
            10.0  // page_size
        )
    }  // adjustment

    fn hpf_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            1.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            0.1,  // step_increment
            1.0,  // page_increment
            10.0  // page_size
        )
    }  // adjustment

    fn var_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            1.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            0.1,  // step_increment
            1.0,  // page_increment
            10.0  // page_size
        )
    }
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

        // TODO restore suppressed entry name for the moment
        // main_box.append(&name);
        // main_box.append(&add);
        // main_box.append(&remove);
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
                let hbox = gtk::Box::builder()
                    .orientation(gtk::Orientation::Horizontal)
                    .spacing(5)
                    .halign(gtk::Align::Center)
                    .build();

                let nuc_grid = gtk::Grid::builder()
                    .row_spacing(5)
                    .column_spacing(5)
                    .build();

                let eqs_label = gtk::Label::new(Some("Eqs"));

                let eqs_spinbtn = gtk::SpinButton::builder()
                    .wrap(true)
                    .adjustment(&Self::eqs_adjustment())
                    .climb_rate(0.5)
                    .digits(0)
                    .orientation(gtk::Orientation::Vertical)
                    .build();

                nuc_grid.attach(&eqs_label, 0, 0, 1, 1);
                nuc_grid.attach(&eqs_spinbtn, 0, 1, 1, 1);

                // "value" is the target_property on the target object
                item.bind_property("eqs", &eqs_spinbtn, "value")
                    .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                    .build();

                // SPIN

                let spin_label = gtk::Label::new(Some("Spin"));

                let spinval_spinbtn = gtk::SpinButton::builder()
                    .wrap(true)
                    .adjustment(&Self::spin_adjustment())
                    .climb_rate(0.5)
                    .digits(1)
                    .orientation(gtk::Orientation::Vertical)
                    .build();

                nuc_grid.attach(&spin_label, 1, 0, 1, 1);
                nuc_grid.attach(&spinval_spinbtn, 1, 1, 1, 1);

                let spinval_spinbtn = gtk::SpinButton::new(Some(&Self::spin_adjustment()), 0.5, 1);
                item.bind_property("spinval", &spinval_spinbtn, "value")
                    .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                    .build();

                // Suppressed
                let spinvar_spinbtn = gtk::SpinButton::new(Some(&Self::var_adjustment()), 0.1, 1);
                item.bind_property("spinvar", &spinvar_spinbtn, "value")
                    .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                    .build();

                // Hpf

                let hpfval_label = gtk::Label::new(Some("Hpf val"));
                let hpfvar_label = gtk::Label::new(Some("Hpf var"));

                let hpfval_spinbtn = gtk::SpinButton::builder()
                    .wrap(true)
                    .adjustment(&Self::hpf_adjustment())
                    .climb_rate(0.1)
                    .digits(1)
                    .orientation(gtk::Orientation::Vertical)
                    .build();

                nuc_grid.attach(&hpfval_label, 2, 0, 1, 1);
                nuc_grid.attach(&hpfval_spinbtn, 2, 1, 1, 1);


                let hpfvar_spinbtn = gtk::SpinButton::builder()
                    .wrap(true)
                    .adjustment(&Self::var_adjustment())
                    .climb_rate(0.1)
                    .digits(1)
                    .orientation(gtk::Orientation::Vertical)
                    .build();

                nuc_grid.attach(&hpfvar_label, 3, 0, 1, 1);
                nuc_grid.attach(&hpfvar_spinbtn, 3, 1, 1, 1);

                item.bind_property("hpfval", &hpfval_spinbtn, "value")
                    .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                    .build();

                item.bind_property("hpfvar", &hpfvar_spinbtn, "value")
                    .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                    .build();

                hbox.append(&nuc_grid);
                let result = hbox.ancestor(gtk::Widget::static_type());
                result.unwrap()
            }
        );

        scroller.set_child(Some(&list_box));

        NucFactoryWidgets { main_box }
    }

    fn view(&mut self, _model: &NucFactoryModel, _sender: Sender<NucParMsg>) {
        // Do things, maybe?
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
        }
    }

    fn var_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            0.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            1.0,  // step_increment
            10.0,  // page_increment
            100.0  // page_size
        )
    }  // adjustment

    fn lwa_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            1.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            1.0,  // step_increment
            10.0,  // page_increment
            100.0  // page_size
        )
    }  // adjustment

    fn amount_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            100.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            10.0,  // step_increment
            100.0,  // page_increment
            1000.0  // page_size
        )
    }  // adjustment

    fn lrtz_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            50.0,  // value
            0.0,  // lower
            100000000.0,  // upper
            10.0,  // step_increment
            100.0,  // page_increment
            1000.0  // page_size
        )
    }  // adjustment

    fn dh1_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(
            0.0,  // value
            -1000.0,  // lower
            100000000.0,  // upper
            10.0,  // step_increment
            100.0,  // page_increment
            1000.0  // page_size
        )
    }  // adjustment

    fn get_as_rad(&self) -> Radical {
        let nucs = match self.nuc_factory.model() {
            Ok(model) => { model.collect_nucs() }
            Err(_) => {
                // TODO Send error
                    Vec::new()
                }
            };

        Radical {
            lwa: Param::set(self.lwa_val, self.lwa_var),
            lrtz: Param::set(self.lrtz_val, self.lrtz_var),
            amount: Param::set(self.amount_val, self.amount_var),
            dh1: Param::set(self.dh1_val, self.dh1_var),
            nucs,
        }
    }

    fn from_rad(&mut self, rad: &Radical) {
        self.lwa_val = rad.lwa.val;
        self.lwa_var = rad.lwa.var;
        self.lrtz_val = rad.lrtz.val;
        self.lrtz_var = rad.lrtz.var;
        self.amount_val = rad.amount.val;
        self.amount_var = rad.amount.var;
        self.dh1_val = rad.dh1.val;
        self.dh1_var = rad.dh1.var;

        // Set nuc values for every single nuc in the model
        match self.nuc_factory.model() {
            Ok(nuc_model) => {
                let howmany_rad_nucs = rad.nucs.len() as usize;
                let howmany_factory_nucs = nuc_model.store.n_items() as usize;

                // Realign factory objects and source
                if howmany_factory_nucs < howmany_rad_nucs {
                    // Add missing nucs to the factory
                    for _i in howmany_factory_nucs..howmany_rad_nucs {
                        nuc_model.store.append(&NucObject::new());
                    }
                } else if howmany_rad_nucs < howmany_factory_nucs {
                    // Remove extra nucs from the factory
                    for _i in howmany_rad_nucs..howmany_factory_nucs {
                        // Del last nuc object
                        nuc_model.store.remove(nuc_model.store.n_items() - 1);
                    }
                }

                // Double checking
                if rad.nucs.len() as usize == nuc_model.store.n_items() as usize {
                    // Set nucs
                    for (index, nuc) in rad.nucs.iter().enumerate() {
                        match nuc_model.store.item(index as u32) {
                            Some(obj) => {
                                // Move this to a method
                                obj.set_property("eqs", nuc.eqs.val as f32);
                                obj.set_property("spinval", nuc.spin.val as f32);
                                obj.set_property("spinvar", nuc.spin.var as f32);
                                obj.set_property("hpfval", nuc.hpf.val as f32);
                                obj.set_property("hpfvar", nuc.hpf.var as f32);
                            }
                            None => {
                                // No objects
                                // Could happen, we don't need any error here;
                            }
                        }
                    }
                } else {
                    // TODO raise error
                }
            }
            Err(e) => {
                println!("Nuc model not found; err: {:?}", e);
            }
        }
    } // from rad
}

#[derive(Debug)]
pub enum RadParMsg {
    AddFirst,
    RemoveLast,
    Update,
    Import(Vec<Radical>),  // TODO Sync?
    _CountAt(WeakDynamicIndex),
    RemoveAt(WeakDynamicIndex),
    _InsertBefore(WeakDynamicIndex),
    _InsertAfter(WeakDynamicIndex),
    SetLwaVal(WeakDynamicIndex, f64),
    SetLwaVar(WeakDynamicIndex, f64),
    SetLrtzVal(WeakDynamicIndex, f64),
    SetLrtzVar(WeakDynamicIndex, f64),
    SetAmountVal(WeakDynamicIndex, f64),
    SetAmountVar(WeakDynamicIndex, f64),
    SetDh1Val(WeakDynamicIndex, f64),
    SetDh1Var(WeakDynamicIndex, f64),
    AddNuc(WeakDynamicIndex, String),
    RemoveLastNuc(WeakDynamicIndex),
}

pub struct RadParModel {
    pars: FactoryVecDeque<RadPar>,
    received_messages: u8,
}

impl Model for RadParModel {
    type Msg = RadParMsg;
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
                let mut new_rads: Vec<Radical> = Vec::new();
                for rad_par in self.pars.iter() {
                    new_rads.push(rad_par.get_as_rad());
                }

                send!(parent_sender, AppMsg::UpdateRads(new_rads));
            }
            // TODO abstract the logic in an external function
            RadParMsg::Import(rads) => {
                // Add the right amount of radical from a source
                let target_len = rads.len();
                if self.pars.len() != target_len {
                    for i in 0..target_len {
                        let mut new_par = RadPar::new(i as u8);
                        new_par.from_rad(&rads[i]);
                        self.pars.push_front(new_par);
                    }
                } else {
                    for i in 0..target_len {
                        self.pars.get_mut(i).expect("Failed getting par").from_rad(&rads[i]);
                    }
                }
            }
            RadParMsg::_CountAt(weak_index) => {
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
            RadParMsg::_InsertBefore(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    self.pars.insert(
                        index.current_index(),
                        RadPar::new(self.received_messages),
                    );
                }
            }
            RadParMsg::_InsertAfter(weak_index) => {
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
            RadParMsg::AddNuc(weak_index, val) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        let nuc_sender = counter.nuc_factory.sender();
                        send!(nuc_sender, NucParMsg::Add(val));
                    }
                }
            }
            RadParMsg::RemoveLastNuc(weak_index) => {
                if let Some(index) = weak_index.upgrade() {
                    if let Some(counter) = self.pars.get_mut(index.current_index()) {
                        let nuc_sender = counter.nuc_factory.sender();
                        send!(nuc_sender, NucParMsg::RemoveLast);
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
            set_spacing: 5,
            append: entries_frame = &gtk::Frame {
                // set_margin_top: 5,
                // set_margin_bottom: 5,
                // set_margin_start: 5,
                // set_margin_end: 5,
                set_child = Some(&gtk::Box) {
                    set_margin_top: 5,
                    set_margin_bottom: 5,
                    set_margin_start: 5,
                    set_margin_end: 5,
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    
                    append: par_head = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 5,
                        set_margin_bottom: 15,
                        // set_halign: gtk::Align::Center,

                        // Don't need this button either
                        //
                        // append: ins_above_button = &gtk::Button {
                        //    set_label: "Add above",
                        //    connect_clicked(sender, key) => move |_| {
                        //        send!(sender, RadParMsg::InsertBefore(key.downgrade()));
                        //    }
                        // },

                        // Dont' need this either
                        // It may be useful in notebook though
                        //
                        // append: ins_below_button = &gtk::Button {
                        //    set_label: "Add below",
                        //    connect_clicked(sender, key) => move |_| {
                        //        send!(sender, RadParMsg::InsertAfter(key.downgrade()));
                        //    }
                        // },
                    },
                    append: rad_params_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 5,
                        set_homogeneous: true,

                        append: left_box = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 5,
                            append: left_head = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_halign: gtk::Align::Center,
                                set_spacing: 5,
                                append: rad_name_label = &gtk::Label {
                                    set_label: "Radical",
                                    set_css_classes: &["heading", "h4"],
                                },

                                // TODO entry function to change rad name
                                // Until that moment, we don't need this button
                                //
                                // append: counter_button = &gtk::Button {
                                //   set_label: watch!(&self.value.to_string()),
                                //    connect_clicked(sender, key) => move |_| {
                                //        send!(sender, RadParMsg::CountAt(key.downgrade()));
                                //    }
                                // },

                                append: remove_button = &gtk::Button {
                                    // set_label: "Remove",
                                    set_icon_name: "user-trash-symbolic",
                                    // set_css_classes: &["flat"],
                                    // TODO Pop up a dialog with a really destructive button
                                    // set_css_classes: &["destructive-action"],
                                    connect_clicked(sender, key) => move |_| {
                                        send!(sender, RadParMsg::RemoveAt(key.downgrade()));
                                    }
                                },

                            },
                            append = &adw::Bin {
                                set_child = Some(&gtk::Grid) {
                                    set_row_spacing: 5,
                                    set_column_spacing: 5,
                                    set_halign: gtk::Align::Center,
                                    attach(0, 0, 1, 1): lwa_label = &gtk::Label {
                                        set_label: "Line Width",
                                        set_halign: gtk::Align::Start,
                                    },
                                    // "next_to" allows to maintain more flexibility for future movements
                                    attach_next_to(Some(&lwa_label), gtk::PositionType::Right, 1, 1): lwa_entry_val =
                                        &gtk::SpinButton {
                                            set_adjustment: &RadPar::lwa_adjustment(),
                                            set_digits: 1,
                                            set_value: watch!(self.lwa_val),
                                            connect_value_changed(sender, key) => move |val| {
                                                send!(sender, RadParMsg::SetLwaVal(key.downgrade(), val.value()));
                                            }
                                        },
                                    attach_next_to(Some(&lwa_entry_val), gtk::PositionType::Right, 1, 1): lwa_entry_var =
                                        &gtk::SpinButton {
                                            set_adjustment: &RadPar::var_adjustment(),
                                            set_digits: 1,
                                            set_climb_rate: 0.5,
                                            set_value: watch!(self.lwa_var),
                                            connect_value_changed(sender, key) => move |val| {
                                                send!(sender, RadParMsg::SetLwaVar(key.downgrade(), val.value()));
                                            }
                                        },
                                    attach(0, 1, 1, 1): lrtz_label = &gtk::Label {
                                        set_label: "Shape (Lrtz/Gauss)",
                                        set_halign: gtk::Align::Start,
                                        set_margin_end: 15,
                                    },
                                    attach(1, 1, 1, 1): lrtz_entry_val = &gtk::SpinButton {
                                        set_adjustment: &RadPar::lrtz_adjustment(),
                                        set_digits: 1,
                                        set_value: watch!(self.lrtz_val),
                                        connect_value_changed(sender, key) => move |val| {
                                            send!(sender, RadParMsg::SetLrtzVal(key.downgrade(), val.value()));
                                        }
                                    },
                                    attach(2, 1, 1, 1): lrtz_entry_var = &gtk::SpinButton {
                                        set_adjustment: &RadPar::var_adjustment(),
                                        set_digits: 1,
                                        set_value: watch!(self.lrtz_var),
                                        set_climb_rate: 0.5,
                                        connect_value_changed(sender, key) => move |val| {
                                            send!(sender, RadParMsg::SetLrtzVar(key.downgrade(), val.value()));
                                        }
                                    },
                                    attach(0, 2, 1, 1): amount_label = &gtk::Label {
                                        set_label: "Amount (%)",
                                        set_halign: gtk::Align::Start,
                                    },
                                    attach(1, 2, 1, 1): amount_entry_val = &gtk::SpinButton {
                                        set_adjustment: &RadPar::amount_adjustment(),
                                        set_digits: 1,
                                        set_value: watch!(self.amount_val),
                                        connect_value_changed(sender, key) => move |val| {
                                            send!(sender, RadParMsg::SetAmountVal(key.downgrade(), val.value()));
                                        }
                                    },
                                    attach(2, 2, 1, 1): amount_entry_var = &gtk::SpinButton {
                                        set_adjustment: &RadPar::var_adjustment(),
                                        set_digits: 1,
                                        set_value: watch!(self.amount_var),
                                        set_climb_rate: 0.5,
                                        connect_value_changed(sender, key) => move |val| {
                                            send!(sender, RadParMsg::SetAmountVar(key.downgrade(), val.value()));
                                        }
                                    },
                                    attach(0, 3, 1, 1): dh1_label = &gtk::Label {
                                        set_label: "Center",
                                        set_halign: gtk::Align::Start,
                                    },
                                    attach(1, 3, 1, 1): dh1_entry_val = &gtk::SpinButton {
                                        set_adjustment: &RadPar::dh1_adjustment(),
                                        set_digits: 1,
                                        set_value: watch!(self.dh1_val),
                                        connect_value_changed(sender, key) => move |val| {
                                            send!(sender, RadParMsg::SetDh1Val(key.downgrade(), val.value()));
                                        }
                                    },
                                    attach(2, 3, 1, 1): dh1_entry_var = &gtk::SpinButton {
                                        set_adjustment: &RadPar::var_adjustment(),
                                        set_digits: 1,
                                        set_value: watch!(self.dh1_var),
                                        set_climb_rate: 0.5,
                                        connect_value_changed(sender, key) => move |val| {
                                            send!(sender, RadParMsg::SetDh1Var(key.downgrade(), val.value()));
                                        }
                                    },
                                },  // Grid
                            },

                        },

                        append: right_box = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 5,
                            append: right_head = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_halign: gtk::Align::Center,
                                set_spacing: 5,

                                append: nuc_controls_buttonbox = &gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 5,
                                    set_halign: gtk::Align::Center,
                                    append: add_new_nuc = &gtk::Button {
                                        set_label: "Add Nucleus",
                                        // set_icon_name: "insert-object-symbolic",
                                        connect_clicked(sender, key) => move |_| {
                                            send!(sender, RadParMsg::AddNuc(key.downgrade(), "New Nucleus".into()));
                                        }
                                    },
                                    append: remove_last_nuc = &gtk::Button {
                                        set_label: "Remove last Nuc",
                                        // set_icon_name: "list-remove-symbolic",
                                        connect_clicked(sender, key) => move |_| {
                                            send!(sender, RadParMsg::RemoveLastNuc(key.downgrade()));
                                        }
                                    },
                                },
                            },
                            append: nuc_entries_frame = &gtk::Frame {
                                set_label_widget = Some(&gtk::Label) {
                                    set_label: "Nuclei",
                                    set_css_classes: &["heading", "h4"],
                                },
                                set_margin_top: 5,
                                set_margin_bottom: 5,
                                set_margin_start: 5,
                                set_margin_end: 5,
                                set_child = Some(&gtk::Box) {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 5,
                                    append: self.nuc_factory.root_widget(),
                                }
                            },
                            append: right_footer = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_halign: gtk::Align::Center,
                                set_spacing: 5,
                                
                            }
                        }
                    },
                },
            },
        }  // Box
    }

    fn pre_view() {
        // widgets.nucs_listbox.append(self.nucs.last().unwrap().root_widget());
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
            .valign(gtk::Align::Center)
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

        let button_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .margin_end(5)
            .margin_top(5)
            .margin_start(5)
            .margin_bottom(5)
            .halign(gtk::Align::End)
            .spacing(5)
            .build();

        // Append new radical to the vector
        let add = gtk::Button::with_label("Add Rad");
        add.set_halign(gtk::Align::Center);

        // Not added to the UI
        let remove = gtk::Button::with_label("Remove Rad");
        remove.set_css_classes(&["destructive-action"]);

        // Convert UI data in simulator parameters
        let update = gtk::Button::with_label("Update Simulator");
        // update.set_css_classes(&["suggested-action"]);

        // TODO cancel function
        // let cancel = gtk::Button::with_label("Cancel");

        main_box.append(&gen_box);

        main_box.append(&add);

        // This button removes the last radical of the vector
        // Functional but suppressed
        // You can recover it, if you need it
        // main_box.append(&remove);

        button_box.append(&update);
        main_box.append(&button_box);

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
