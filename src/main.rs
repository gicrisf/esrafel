use adw::{
    prelude::{AdwApplicationWindowExt, BinExt},
    CenteringPolicy, ViewStackPage
};

use gtk::{
    prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt, WidgetExt,
              DrawingAreaExt, Cast, CheckButtonExt, PopoverExt},
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
    ParentWindow,
};

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

mod drawers;
mod esr_io;
mod sim;
mod params;

use sim::Radical;
use drawers::{Line, Color};
use params::ParamsModel;

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
            // background_color: Color::rgb(250.0, 224.0, 55.0),
            // theoretical_color: Color::rgb(149.0, 25.0, 12.0),
            theoretical_color: Color::rgb(230.0, 111.0, 67.0),
            //empirical_color: Color::rgb(0.3, 0.3, 0.3),
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
            set_spacing: 10,
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
            // std::thread::sleep(std::time::Duration::from_millis(20));
            // send!(sender, ChartMsg::Update);
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

// Param Factory Model

// -- Open Button

struct OpenFileButtonConfig {}

impl OpenDialogConfig for OpenFileButtonConfig {
    type Model = AppModel;

    fn open_dialog_config(_model: &Self::Model) -> OpenDialogSettings {
        OpenDialogSettings {
            accept_label: "Open",
            cancel_label: "Cancel",
            create_folders: true,
            is_modal: true,
            filters: Vec::new(),
        }
    }
}

impl OpenButtonConfig for OpenFileButtonConfig {
    fn open_button_config(_model: &Self::Model) -> OpenButtonSettings {
        OpenButtonSettings {
            text: "Open file",
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

// -- AppModel

#[derive(Default)]
struct AppModel {
    empirical: Option<Vec<f64>>,
    rads: Vec<Radical>,
    points: usize,  // TODO: use usize
    sweep: f64,
    sigma: f64,
    iters: usize,
    montecarlo: bool,
}

enum AppMsg {
    IterMontecarlo,
    ToggleMontecarlo(bool),
    Open(PathBuf),
    SetSweep(f64),
    SetPoints(usize),  // then, temporarily convert to f64
}

#[derive(relm4::Components)]
struct AppComponents {
    chart: RelmComponent<ChartModel, AppModel>,
    params: RelmComponent<ParamsModel, AppModel>,
    open_button: RelmComponent<OpenButtonModel<OpenFileButtonConfig>, AppModel>,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::IterMontecarlo => {
                // This is a fast and working solution, but a persistent iteration is not an elegant move
                // Must search for another tracking method, but it's not a priority rn
                if self.montecarlo {
                    if let Some(emp) = &self.empirical {
                    let (newsigma, newrads) = sim::mc_fit(self.rads.clone(), &emp, self.points as f64);

                    // TODO: CONDITIONAL REASSIGNMENT of sigma here!
                    // println!("{:?}", newsigma);
                    self.sigma = newsigma;
                    self.rads = newrads;
                    self.iters+=1;
                    components.chart.send(ChartMsg::AddTheoretical(sim::calcola(&self.rads)))
                                    .expect("Failed sending new theoretical spectrum to the Chart");

                    // println!("iters: {}", self.iters);
                } // if empirical exists
                } // if montecarlo toggled
            }
            AppMsg::ToggleMontecarlo(v) => {
                self.montecarlo = v;
            }  // ./Montecarlo
            AppMsg::Open(path) => {
                println!("* Open file at {:?} *", path);
                let mut data = String::new();
                let mut f = File::open(path).expect("Unable to open file");
                f.read_to_string(&mut data).expect("Unable to read string");

                // TODO manage errors in reading files!
                // TODO choose right function basing on the extension
                // match -> ascii -> json -> esr (etc.)
                let new_empirical_vec = esr_io::get_from_ascii(&data);

                // Store in both AppModel and ChartModel?
                self.empirical = Some(new_empirical_vec.clone());
                components.chart.send(ChartMsg::AddEmpirical(new_empirical_vec))
                                .expect("Failed sending empirical spectrum to the Chart");
            }
            AppMsg::SetSweep(value) => {
                self.sweep = value;
            }
            AppMsg::SetPoints(value) => {
                self.points = value;
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
                append: stack = &adw::ViewStack {
                    set_vexpand: true,
                    add_titled(Some("Params"), "Parameters") = &gtk::Box {
                        set_orientation: Orientation::Vertical,
                        set_hexpand: false,
                        set_spacing: 15,
                        append = general_pars_box = &gtk::Box {
                            set_orientation: Orientation::Horizontal,
                            set_hexpand: true,
                            set_spacing: 15,
                            set_halign: gtk::Align::Fill,
                            append = &gtk::Label {
                                set_label: "General parameters: ",
                            },
                            append: sweep_entry = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 10,
                                set_homogeneous: true,
                                append: &gtk::Label::new(Some("Sweep")),
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
                                set_spacing: 10,
                                set_homogeneous: true,
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
                        },  // ./ general pars box
                        append = &gtk::Separator::new(gtk::Orientation::Horizontal) {
                            set_margin_bottom: 5,
                        },
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
                                        send!(sender, AppMsg::ToggleMontecarlo(v.is_active()))
                                    }
                                }
                            }
                        },


                    } -> plot_page: ViewStackPage {
                        set_icon_name: Some("media-playback-start-symbolic"),
                        set_needs_attention: watch!(model.montecarlo),
                    },
                },
                append: bottom_bar = &adw::ViewSwitcherBar {
                    set_stack: Some(&stack),
                }
            },  // set_content
        } // main_window
    }  // view!

    fn post_init() {
       title
            .bind_property("title-visible", &bottom_bar, "reveal")
            .flags(gtk::glib::BindingFlags::SYNC_CREATE)
            .build();

        // IDEA How to send from thread and through components
        // let chart_sender = components.chart.sender();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(20));
            send!(sender, AppMsg::IterMontecarlo);
            // send!(chart_sender, ChartMsg::Update);
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
        rads: vec![Radical::var_probe()],
        points: 1024,
        sweep: 100.0,
        sigma: 1E+20,
        iters: 0,
        montecarlo: false,
    };
    let app = RelmApp::new(model);
    app.run();
} 
