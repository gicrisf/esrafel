use adw::{
    prelude::{AdwApplicationWindowExt, BinExt},
    CenteringPolicy, ViewStackPage
};

use gtk::{
    prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt, WidgetExt,
              DrawingAreaExt, Cast, CheckButtonExt, PopoverExt, FrameExt, ComboBoxExtManual},
    Orientation,
    cairo::Context,
};

use relm4::{
    adw, gtk, send,
    AppUpdate, RelmComponent, ComponentUpdate, Model, RelmApp, Sender, Widgets,
    // factory::{FactoryVecDeque, DynamicIndex, WeakDynamicIndex},
};

use relm4_components::{
    open_button::{
        OpenButtonConfig, OpenButtonModel, OpenButtonParent, OpenButtonSettings,
    },
    open_dialog::{OpenDialogConfig, OpenDialogSettings},
    save_dialog::{SaveDialogModel, SaveDialogMsg, SaveDialogParent, SaveDialogSettings},
    ParentWindow,
};

use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

use serde::{Serialize, Deserialize};

mod drawers;
mod esr_io;
mod sim;
mod params;
mod nuc_object;

use sim::Radical;
use drawers::{Line, Color};
use params::{RadParModel, RadParMsg};

// -- Chart model

#[derive(Default)]
struct ChartModel {
    width: f64,
    height: f64,
    background_color: Color,
    theoretical_color: Color,
    empirical_color: Color,
    theoretical_line: Option<Line>,
    empirical_line: Option<Line>,
}

enum ChartMsg {
    AddEmpirical(Vec<f64>),
    AddTheoretical(Vec<f64>),
    Resize((i32, i32)),
}

impl Model for ChartModel {
    type Msg = ChartMsg;
    type Widgets = ChartWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ChartModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ChartModel {
            width: 1000.0,
            height: 600.0,
            background_color: Color::rgb(24.0, 24.0, 22.0),
            theoretical_color: Color::rgb(230.0, 111.0, 67.0),
            empirical_color: Color::rgb(254.0, 242.0, 235.0),
            theoretical_line: None,
            empirical_line: None,
        }
    }

    fn update(
        &mut self,
        msg: ChartMsg,
        _components: &(),
        _sender: Sender<ChartMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            // ChartMsg::Demo => {}

            // Maybe NewEmpirical is a more proper name?
            // Maintaining this in the eventyality of a multi-spectrum option
            ChartMsg::AddEmpirical(v) => {
                self.empirical_line = Some(Line::new(v));
            }
            ChartMsg::AddTheoretical(v) => {
                self.theoretical_line = Some(Line::new(v));
            }
            ChartMsg::Resize((x, y)) => {
                self.width = x as f64;
                self.height = y as f64;
            }
        }
    }
}

// -- Chart Widgets

#[relm4::widget]
impl Widgets<ChartModel, AppModel> for ChartWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_hexpand: true,
            append: area = &gtk::DrawingArea {
                set_vexpand: true,
                set_hexpand: true,
                set_content_width: 1000,
                set_content_height: 600,

                connect_resize(sender) => move |_, x, y| {
                    send!(sender, ChartMsg::Resize((x, y)))
                }
            }  // ./DrawingArea
        }
    }  // view!

    additional_fields! {
        handler: relm4::drawing::DrawHandler
    }

    fn post_init() {
        let mut handler = relm4::drawing::DrawHandler::new().unwrap();
        handler.init(&area);

        // Memo: you can spawn a thread here
        //
        // std::thread::spawn(move || loop {
        //  std::thread::sleep(std::time::Duration::from_millis(20));
        //  send!(sender, ChartMsg::Update);
        // });

    }  // post init

    // This draws in loop
    fn pre_view() {
        let cr = self.handler.get_context().unwrap();
        drawers::paint_bg(&cr, &model.background_color);
        if let Some(v) = &model.empirical_line {
            drawers::draw_classic(&cr, &v, model.width, model.height, &model.empirical_color);
        };

        if let Some(v) = &model.theoretical_line {
            drawers::draw_classic(&cr, &v, model.width, model.height, &model.theoretical_color);
        };

        // IDEA: if track!(&model.show_demo) ...
        // Draw noise or choose your opening demo
        // drawers::draw_noise(&cr, &v, model.width, model.height, &model.theoretical_color);
        // A button for opening spectra/parameters would be better
        // (Granite-style)
    }  // pre view
}

// -- Open Button

struct OpenFileButtonConfig {}

impl OpenDialogConfig for OpenFileButtonConfig {
    type Model = AppModel;

    fn open_dialog_config(_model: &Self::Model) -> OpenDialogSettings {

        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.txt");
        filter.add_pattern("*.json");

        OpenDialogSettings {
            accept_label: "Open",
            cancel_label: "Cancel",
            create_folders: true,
            is_modal: true,
            filters: vec![filter],
        }
    }
}

impl OpenButtonConfig for OpenFileButtonConfig {
    fn open_button_config(_model: &Self::Model) -> OpenButtonSettings {
        OpenButtonSettings {
            text: "Open file",
            // TODO move to gresources
            recently_opened_files: Some(".recent_files"),
            max_recent_files: 10,
        }
    }
}

impl OpenButtonParent for AppModel {
    fn open_msg(path: PathBuf) -> Self::Msg {
        AppMsg::Open(path)
    }
}

// Import parameters

struct ImportParsButtonConfig {}

impl OpenDialogConfig for ImportParsButtonConfig {
    type Model = AppModel;

    fn open_dialog_config(_model: &Self::Model) -> OpenDialogSettings {

        let filter = gtk::FileFilter::new();
        // Just JSON files with esrafel head
        filter.add_pattern("*.esrafel");

        OpenDialogSettings {
            accept_label: "Open",
            cancel_label: "Cancel",
            create_folders: true,
            is_modal: true,
            filters: vec![filter],
        }
    }
}

impl OpenButtonConfig for ImportParsButtonConfig {
    fn open_button_config(_model: &Self::Model) -> OpenButtonSettings {
        OpenButtonSettings {
            text: "Import Params.",
            // TODO move to gresources
            recently_opened_files: Some(".recent_states"),
            max_recent_files: 10,
        }
    }
}

// Save pars

struct SaveDialogConfig {}
impl relm4_components::save_dialog::SaveDialogConfig for SaveDialogConfig {
    type Model = AppModel;

    fn dialog_config(_model: &Self::Model) -> SaveDialogSettings {
        SaveDialogSettings {
            accept_label: "Save",
            cancel_label: "Cancel",
            create_folders: true,
            is_modal: true,
            filters: Vec::new(),
        }
    }
}

impl SaveDialogParent for AppModel {
    fn save_msg(path: PathBuf) -> Self::Msg {
        AppMsg::SaveResponse(path)
    }
}

// -- AppModel

// Available simulation methods
// This will make it easier further extensions of thee backend

#[derive(Serialize, Deserialize)]
enum SimulationMethod {
    MC199,
    // Dynamic1999
    // ...
}

#[derive(Default, Serialize, Deserialize)]
struct AppModel {
    empirical: Option<Vec<f64>>,
    rads: Vec<Radical>,
    points: usize,
    sweep: f64,
    sigma: f64,
    iters: usize,
    montecarlo: bool,
    log: Vec<String>,
    sim_method: Option<SimulationMethod>,
    #[serde(skip)]
    last_toast: Option<adw::Toast>,
}

enum AppMsg {
    IterMontecarlo,
    Redraw,
    ToggleMontecarlo(bool),
    Open(PathBuf),
    UpdateRads(Vec<Radical>),
    SetSweep(f64),
    SetPoints(usize),  // then, temporarily convert to f64
    RefreshPanel,
    SpawnToast(String),
    ResetToast,
    SetSimMethod(SimulationMethod),
    SaveRequest,
    SaveResponse(PathBuf),
}

#[derive(relm4::Components)]
struct AppComponents {
    chart: RelmComponent<ChartModel, AppModel>,
    params: RelmComponent<RadParModel, AppModel>,
    open_button: RelmComponent<OpenButtonModel<OpenFileButtonConfig>, AppModel>,
    import_pars_button: RelmComponent<OpenButtonModel<ImportParsButtonConfig>, AppModel>,
    save_dialog: RelmComponent<SaveDialogModel<SaveDialogConfig>, AppModel>,
    // status: RelmComponent<StatusModel, AppModel>,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::UpdateRads(new_rads) => {
                self.rads = new_rads;
                // TODO Remove this, DEBUGGING ONLY
                println!("{:?}", self.rads);

                send!(sender, AppMsg::SpawnToast("Updated".into()));
            }
            AppMsg::RefreshPanel => {
                components.params.send(RadParMsg::Import(self.rads.clone()))
                                 .expect("Refreshing param panel failed");
            }
            AppMsg::IterMontecarlo => {
                // This is a fast and working solution, but a persistent iteration is not an elegant move
                // Must search for another tracking method, but it's not a priority rn
                if self.montecarlo {
                    if let Some(emp) = &self.empirical {

                        let (newsigma, newteor, newrads) =
                            sim::mc_fit(
                                &emp,
                                self.points as f64,
                                self.sweep,
                                self.sigma,
                                self.rads.clone(),
                            );

                        self.sigma = newsigma;
                        self.rads = newrads;

                        components.chart.send(ChartMsg::AddTheoretical(newteor))
                                        .expect("Failed sending new theoretical spectrum to the Chart");

                        // Randomize parameters for next iteration
                        // self.newrads = sim::caso(&self.rads);

                        self.iters+=1;
                    } // if empirical exists
                } // if montecarlo toggled
            }
            AppMsg::Redraw => {
                // TODO it shouldn't call directly the simulator function
                // Must be compatible with every future simulator implementation
                if self.montecarlo {
                    components.chart.send(ChartMsg::AddTheoretical(
                        sim::calcola(&self.rads, self.sweep, self.points as f64)))
                                    .expect("Failed sending new theoretical spectrum to the Chart");
                }
            }
            AppMsg::ToggleMontecarlo(is_going) => {
                self.montecarlo = is_going;
            }  // ./Montecarlo
            AppMsg::Open(path) => {
                let mut data = String::new();

                if let Some(ext) = path.extension() {
                    let ext_as_str =
                        ext.to_str().expect("Cannot convert extension to string; non-unicode chars, maybe?");

                    match ext_as_str {
                        "txt" => {
                            let f: Option<File>;

                            match File::open(path) {
                                Ok(file) => f = Some(file),
                                Err(e) => {
                                    let err_string = format!("Unable to open txt file. Error: {}", e);
                                    send!(sender, AppMsg::SpawnToast(err_string.into()));
                                    f = None;
                                }
                            };

                            match f {
                                Some(mut file) => {
                                    match file.read_to_string(&mut data) {
                                        Ok(_) => {
                                            self.empirical = Some(esr_io::get_from_ascii(&data));
                                            send!(sender, AppMsg::SpawnToast("Loaded!".into()));
                                        },
                                        Err(e) => {
                                            let err_string = format!("Unable to read string in this file. Error: {}", e);
                                            send!(sender, AppMsg::SpawnToast(err_string.into()));
                                        }
                                    }
                                }
                                None => {
                                    // Do nothing
                                }
                            }
                        } // txt case
                        "esr" => {
                            send!(sender, AppMsg::SpawnToast("Legacy format not supported yet!".into()));
                        }
                        "json" => {
                            send!(sender, AppMsg::SpawnToast("JSON format not supported yet!".into()));
                        }  // json case
                        "esrafel" => {
                            send!(sender, AppMsg::SpawnToast("State successfully imported!".into()));
                        }
                        _ => {
                            send!(sender, AppMsg::SpawnToast("How did you even clicked on this file?!".into()));
                        }
                    }
                }

                // Store in both AppModel and ChartModel?
                if let Some(emp) = &self.empirical {
                    components.chart.send(ChartMsg::AddEmpirical(emp.to_vec()))
                                    .expect("Failed sending empirical spectrum to the Chart");
                }
            }
            AppMsg::SetSweep(value) => {
                self.sweep = value;
            }
            AppMsg::SetPoints(value) => {
                self.points = value;
            }
            AppMsg::SpawnToast(msg) => {
                self.last_toast = Some(adw::Toast::new(&msg));
                self.log.push(msg);
            }
            AppMsg::ResetToast => {
                self.last_toast = None;
            }
            AppMsg::SetSimMethod(method) => {
                self.sim_method = Some(method);
            }
            AppMsg::SaveRequest => {
                components
                    .save_dialog
                    .send(SaveDialogMsg::SaveAs(".esrafel".into()))
                    .unwrap();
            }
            AppMsg::SaveResponse(path) => {
                // Serialize model
                let data = serde_json::to_string(&self).unwrap();

                // Write to file
                match File::create(path.clone()) {
                    Ok(mut file) => {
                        match write!(file, "{}", &data) {
                            Ok(_) => {
                                send!(sender, AppMsg::SpawnToast(
                                    format!("File successfully saved into {:?}", &path).into()
                                ));
                            }
                            Err(e) => {
                                let err_string = format!("Unable to create file. Error: {}", e);
                                send!(sender, AppMsg::SpawnToast(err_string.into()));
                            }
                        }
                    }
                    Err(e) => {
                        let err_string = format!("Unable to create file. Error: {}", e);
                        send!(sender, AppMsg::SpawnToast(err_string.into()));
                    }
                };

                // Done
            }
        }
        true
    }
}

// -- AppWidgets

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_default_width: 450,
            set_default_height: 200,
            set_content = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    pack_start: components.open_button.root_widget(),
                    set_title_widget: title = Some(&adw::ViewSwitcherTitle) {
                        set_title: "Esrafel",
                        set_stack: Some(&stack),
                    },
                    pack_end: menu_button = &gtk::MenuButton::new() {
                        set_icon_name: "open-menu-symbolic",
                        set_popover: popover = Some(&gtk::Popover) {
                            set_child = Some(&gtk::Box) {
                                set_orientation: gtk::Orientation::Vertical,
                                append: timer = &gtk::CheckButton::with_label("30s") {
                                    connect_toggled(sender) => move |b| {
                                        if b.is_active() {
                                        }
                                    }
                                },
                                append = &gtk::CheckButton::with_label("60s") {
                                    set_group: Some(&timer),
                                    connect_toggled(sender) => move |b| {
                                        if b.is_active() {
                                        }
                                    }
                                },
                                append = &gtk::CheckButton::with_label("180s") {
                                    set_group: Some(&timer),
                                    connect_toggled(sender) => move |b| {
                                        if b.is_active() {
                                        }
                                    }
                                }
                            }
                        }  // ./popover
                    },
                    set_centering_policy: CenteringPolicy::Strict,
                },
                append: body = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    // append: status_bar = &adw::Bin {
                        // set_child: Some(components.status.root_widget())
                    // },
                    append: toast_overlay = &adw::ToastOverlay {
                        set_child: stack = Some(&adw::ViewStack) {
                            set_vexpand: true,
                            add_titled(Some("Params"), "Parameters") = &gtk::Box {
                                set_orientation: Orientation::Vertical,
                                set_hexpand: false,
                                set_spacing: 5,
                                append = general_pars_box = &gtk::Box {
                                    set_orientation: Orientation::Horizontal,
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Center,
                                    set_margin_top: 5,
                                    set_margin_start: 5,
                                    set_spacing: 5,
                                    append = &gtk::Frame {
                                        // set_label: Some("General"),
                                        // TODO remove this request
                                        // set_width_request: 715,
                                        set_child = Some(&gtk::Box) {
                                            append: open_params = &gtk::Box {
                                                set_orientation: gtk::Orientation::Horizontal,
                                                set_spacing: 5,
                                                append = &gtk::Box {
                                                    set_orientation: gtk::Orientation::Horizontal,
                                                    set_spacing: 5,
                                                    set_margin_start: 5,
                                                    set_margin_end: 5,
                                                    set_margin_top: 5,
                                                    set_margin_bottom: 5,
                                                    append: components.import_pars_button.root_widget(),
                                                },
                                                append = &gtk::Box {
                                                    set_orientation: gtk::Orientation::Horizontal,
                                                    set_spacing: 5,
                                                    set_margin_start: 5,
                                                    set_margin_end: 5,
                                                    set_margin_top: 5,
                                                    set_margin_bottom: 5,

                                                    append = &gtk::Button {
                                                        set_label: "Export Params.",
                                                        // set_icon_name: "document-save-symbolic",
                                                        connect_clicked(sender) => move |_| {
                                                            send!(sender, AppMsg::SaveRequest);
                                                        },
                                                    },
                                                },
                                            },
                                            append: sweep_entry = &gtk::Box {
                                                set_orientation: gtk::Orientation::Horizontal,
                                                set_spacing: 5,
                                                set_margin_start: 5,
                                                set_margin_end: 5,
                                                set_margin_top: 5,
                                                set_margin_bottom: 5,
                                                // set_homogeneous: true,
                                                append = &gtk::Label {
                                                    set_text: "Sweep",
                                                },
                                                append: sweep_spin = &gtk::SpinButton {
                                                    set_adjustment: &gtk::Adjustment::new(
                                                        model.sweep,  // value
                                                        0.0,  // lower
                                                        100000000.0,  // upper
                                                        10.0,  // step_increment
                                                        100.0,  // page_increment
                                                        1000.0  // page_size
                                                    ),
                                                    connect_value_changed(sender) => move |val| {
                                                        send!(sender, AppMsg::SetSweep(val.value()))
                                                    }
                                                },
                                            },
                                            append: points_entry = &gtk::Box {
                                                set_orientation: gtk::Orientation::Horizontal,
                                                set_spacing: 5,
                                                set_margin_top: 5,
                                                set_margin_bottom: 5,
                                                set_margin_start: 5,
                                                set_margin_end: 5,
                                                // set_homogeneous: true,
                                                append: &gtk::Label::new(Some("Points")),
                                                append: points_spin = &gtk::SpinButton {
                                                    set_adjustment: &gtk::Adjustment::new(
                                                        model.points as f64,  // value
                                                        0.0,  // lower
                                                        100000000.0,  // upper
                                                        1.0,  // step_increment
                                                        10.0,  // page_increment
                                                        1000.0  // page_size
                                                    ),
                                                    connect_value_changed(sender) => move |val| {
                                                        send!(sender, AppMsg::SetPoints(val.value_as_int() as usize));
                                                    }
                                                },
                                            },
                                            append = &gtk::Box {
                                                set_orientation: gtk::Orientation::Horizontal,
                                                set_spacing: 5,
                                                set_margin_start: 5,
                                                set_margin_end: 5,
                                                set_margin_top: 5,
                                                set_margin_bottom: 5,
                                                append: &gtk::Label::new(Some("Simulation method")),
                                                append: sim_method_entry = &gtk::ComboBoxText {
                                                    append_text: "MC 1999"
                                                },  // toggle button
                                            },

                                        }
                                    },

                                },  // ./ general pars box
                                append = &gtk::ScrolledWindow {
                                    set_hscrollbar_policy: gtk::PolicyType::Never,
                                    set_min_content_height: 360,
                                    set_vexpand: true,
                                    set_child: Some(components.params.root_widget()),
                                },
                            } -> params_page: ViewStackPage {
                                set_icon_name: Some("document-print-symbolic"),
                                set_badge_number: watch!(model.rads.len() as u32),
                            },
                            add_titled(Some("Plot"), "Plot") = &gtk::Box {
                                set_orientation: Orientation::Vertical,
                                set_hexpand: false,

                                // `component!` seems like it's still a not supported macro?
                                // append: component!(Some(chart)),
                                // ALERT, the next line couldn't work in other branches
                                append = &adw::Bin {
                                    set_child: Some(components.chart.root_widget()),
                                },

                                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {
                                    set_margin_bottom: 5,
                                },

                                append = &adw::Bin {
                                    set_margin_bottom: 5,
                                    set_child = Some(&gtk::CenterBox) {
                                        set_center_widget = Some(&gtk::ToggleButton) {
                                            set_label: "Run MonteCarlo",
                                            set_active: model.montecarlo,
                                            connect_clicked(sender) => move |v| {
                                                let is_mc_active = v.is_active();
                                                send!(sender, AppMsg::ToggleMontecarlo(is_mc_active));

                                                // Refresh Parameters in GUI when you stop running MC
                                                if !is_mc_active {
                                                    send!(sender, AppMsg::RefreshPanel)
                                                };
                                            }
                                        }
                                    }
                                },

                            } -> plot_page: ViewStackPage {
                                set_icon_name: Some("media-playback-start-symbolic"),
                                set_needs_attention: watch!(model.montecarlo),
                            },
                        },
                    },
                    // append: stack = &adw::ViewStack {  },
                },
                // TODO Explore this `bottom_bar`
                append: bottom_bar = &adw::ViewSwitcherBar {
                    set_stack: Some(&stack),
                },

            },  // set_content
        } // main_window
    }  // view!

    fn pre_view() {
        if let Some(toast) = &model.last_toast {
            self.toast_overlay.add_toast(&toast);
            send!(sender, AppMsg::ResetToast);
        }

        match &model.sim_method {
            Some(method) => {
                match method {
                    SimulationMethod::MC199 => {
                        self.sim_method_entry.set_active(Some(0));
                    }
                }
            }
            None => {
                // Do nothing except:
                // Invite choosing a method
                // send!(sender, AppMsg::SpawnToast("Choose a simulation method!".into()));
                //
                // Leaved as is, this could never start the actual GUI
                // Considering the fact that this is basically unreachable
                // it's better not to overcomplicate this section
            }
        }
    }

    fn post_init() {
       title
            .bind_property("title-visible", &bottom_bar, "reveal")
            .flags(gtk::glib::BindingFlags::SYNC_CREATE)
            .build();

        send!(sender, AppMsg::RefreshPanel);

        // IDEA How to send from thread and through components
        // let chart_sender = components.chart.sender();

        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(40));
            send!(sender, AppMsg::IterMontecarlo);
            std::thread::sleep(std::time::Duration::from_millis(40));
            send!(sender, AppMsg::Redraw);
        });
    }
}

impl ParentWindow for AppWidgets {
    fn parent_window(&self) -> Option<gtk::Window> {
        Some(self.main_window.clone().upcast::<gtk::Window>())
    }
}

// -- MAIN

fn main() {
    let model = AppModel {
        empirical: None,
        // TODO start without radicals and show welcome screen
        rads: vec![Radical::var_probe()],
        points: 1024,
        sweep: 100.0,
        sigma: 100000000000000000000.0,  //1e+20
        iters: 0,
        montecarlo: false,
        last_toast: None,
        log: Vec::new(),
        sim_method: Some(SimulationMethod::MC199),
    };
    let app = RelmApp::new(model);
    app.run();
} 
