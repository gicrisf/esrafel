use adw::{
    prelude::{AdwApplicationWindowExt},
    CenteringPolicy, ViewStackPage
};

use gtk::{
    prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt, WidgetExt,
              DrawingAreaExt, Cast},
    Orientation,
    cairo::{Context},
};

use relm4::{
    adw, gtk, send,
    AppUpdate, RelmComponent, ComponentUpdate, Model, RelmApp, Sender, Widgets,
    // factory::FactoryVecDeque,
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

use drawers::{Line, Color};

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
    Update,
    AddEmpirical(Vec<f64>),
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
            background_color: Color::rgb(250.0, 224.0, 55.0),
            theoretical_color: Color::rgb(0.3, 0.3, 0.3),
            empirical_color: Color::rgb(0.3, 0.3, 0.3),
            // theoretical_line: Some(
                // TODO Change to None and pass value from the main model
                // Line::new(dsp::generator::noise(1024, 20.0, 8).data.iter().map(|&x| x as f64).collect::<Vec<_>>())
                // Line::new(sim::calcola(vec![sim::Radical::probe()])),
            // ),
            theoretical_line: None,
            empirical_line: None,
        }
    }

    fn update(
        &mut self,
        msg: ChartMsg,
        _components: &(),
        _sender: Sender<ChartMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            // ChartMsg::Demo => {}
            ChartMsg::Update => {

                // self.line = dsp::generator::noise(1024, 20.0, 8).data;

                /*
                self.theoretical_line = Some(
                    Line::new(dsp::generator::noise(1024, 20.0, 8).data.iter().map(|&x| x as f64).collect::<Vec<_>>())
                );
                */

            }
            ChartMsg::AddEmpirical(v) => {
                self.empirical_line = Some(Line::new(v));
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

        std::thread::spawn(move || loop {
            // std::thread::sleep(std::time::Duration::from_millis(20));
            // send!(sender, ChartMsg::Update);
        });
    }  // post init

    // This draws in loop
    fn pre_view() {
        let cr = self.handler.get_context().unwrap();
        drawers::paint_bg(&cr, &model.background_color);
        if let Some(v) = &model.empirical_line {
            drawers::draw_classic(&cr, &v, model.width, model.height, &model.empirical_color);
        };

        if let Some(v) = &model.theoretical_line {
            // drawers::draw_noise(&cr, &v, model.width, model.height, &model.theoretical_color);
            drawers::draw_classic(&cr, &v, model.width, model.height, &model.theoretical_color);
        };
    }  // pre view
}

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
    theoretical: Option<Vec<f64>>,
    // points: f64,
    // sweep: f64,
    // params: FactoryVecDeque<Radical>,
    // sigma: f64,
    // iters: usize,
    montecarlo: bool,
}

enum AppMsg {
    Montecarlo(bool),
    Open(PathBuf),
}

#[derive(relm4::Components)]
struct AppComponents {
    chart: RelmComponent<ChartModel, AppModel>,
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
            AppMsg::Montecarlo(v) => {
                self.montecarlo = v;
            }  // Montecarlo
            AppMsg::Open(path) => {
                println!("* Open file at {:?} *", path);
                let mut data = String::new();
                let mut f = File::open(path).expect("Unable to open file");
                f.read_to_string(&mut data).expect("Unable to read string");
                // println!("{}", data);

                // TODO manage errors in reading files!
                // self.empirical = Some(esr_io::get_from_ascii(&data));
                components.chart.send(ChartMsg::AddEmpirical(esr_io::get_from_ascii(&data)))
                                .expect("Failed sending empirical spectrum to the Chart");
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
                    set_centering_policy: CenteringPolicy::Strict,
                },
                append: stack = &adw::ViewStack {
                    set_vexpand: true,
                    add_titled(Some("Params"), "Parameters") = &gtk::Box {
                        set_orientation: Orientation::Vertical,
                        set_hexpand: false,
                        append = &gtk::Label {
                            set_label: "This is the parameters page",
                        },
                        append = &gtk::Button {
                            set_label: "Increase",
                            connect_clicked(sender) => move |_| {
                                // send!(sender, AppMsg::Increment)
                            }
                        },
                        append = &gtk::Button {
                            set_label: "Decrease",
                            connect_clicked(sender) => move |_| {
                                // send!(sender, AppMsg::Decrement)
                            }
                        },
                    } -> params_page: ViewStackPage {
                        set_icon_name: Some("document-print-symbolic"),
                        // set_badge_number: watch!(model.counter as u32),
                    },
                    add_titled(Some("Plot"), "Plot") = &gtk::Box {
                        set_orientation: Orientation::Vertical,
                        set_hexpand: false,

                        // TODO choose if adding plot-related status here

                        // `component!` seems like it's still a not supported macro?
                        // append: component!(Some(chart)),

                        // ALERT, the next line couldn't work in other branches
                        append: components.chart.root_widget(),
                        append = &gtk::ToggleButton {
                            set_label: "Run MonteCarlo",
                            set_active: model.montecarlo,
                            connect_clicked(sender) => move |v| {
                                send!(sender, AppMsg::Montecarlo(v.is_active()))
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
        /*
        let chart_sender = components.chart.sender();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(20));
            send!(chart_sender, ChartMsg::Update);
        });
        */
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
        theoretical: Some(sim::calcola(vec![sim::Radical::probe()])),
        // points: 1024.0,
        // sweep: 100.0,
        // params: FactoryVecDeque::new(),
        // sigma: 1E+20,
        // iters: 0,
        montecarlo: false,
    };
    let app = RelmApp::new(model);
    app.run();
} 
