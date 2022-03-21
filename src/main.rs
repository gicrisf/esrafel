use adw::{
    prelude::{AdwApplicationWindowExt},
    CenteringPolicy, ViewStackPage
};
use gtk::{
    prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt, WidgetExt, DrawingAreaExt, DrawingAreaExtManual},
    Orientation,
};
use relm4::{adw, gtk, send, AppUpdate, RelmComponent, ComponentUpdate, Model, RelmApp, Sender, Widgets};
// use relm4::*;
// use relm4::factory::FactoryVecDeque;

use std::f64::consts::PI;

// -- Entities

pub struct Param {
    pub val: f64,  // Value; starts with 0.0
    pub var: f64,  // Variation; starts with: 0.0
}

pub struct Radical {
    pub lwa: Param,  // Line width A
    // pub lwb: Param,
    // pub lwc: Param,
    pub lrtz: Param,  // Lorentzian linewidth parameter (%)
    pub amount: Param,  // Relative amount
    pub dh1: Param,
    // pub nucs: Vec<Nucleus>,
}

// -- Chart model

#[derive(Default)]
struct ChartModel {
    width: f64,
    height: f64,
}

enum ChartMsg {
    Draw,
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
            width: 100.0,
            height: 100.0,
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
            ChartMsg::Draw => {
                // Draw
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
                set_content_width: 500,
                set_content_height: 500,
                set_draw_func: |_, cr, _, _| {
                    // TODO replace with sinusoidal opening demo
                    // text example adapted from:
                    // https://github.com/gtk-rs/gtk3-rs/blob/master/examples/cairo_test/main.rs

                    // TODO Study how this scaling function works exactly
                    cr.scale(500f64, 500f64);

                    // Yellow background
                    cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
                    cr.paint().expect("Invalid cairo surface state");

                    // Line drawer settings
                    cr.set_line_width(0.03);
                    cr.set_source_rgb(0.3, 0.3, 0.3);

                    // Draw rectangle
                    // cr.rectangle(0.1, 0.1, 1.0, 1.0);
                    // cr.stroke().expect("Invalid cairo surface state");

                    // Draw circle
                    cr.arc(0.6, 0.6, 0.4, 0.0, PI * 2.);
                    cr.stroke().expect("Invalid cairo surface state");
                },

                connect_resize(sender) => move |_, x, y| {
                    send!(sender, ChartMsg::Resize((x, y)))
                }
            }  // ./DrawingArea
        }
    }
}


// -- AppModel

#[derive(Default)]
struct AppModel {
    empirical: Option<Vec<f64>>,
    theoretical: Option<Vec<f64>>,
    points: f64,
    sweep: f64,
    // params: FactoryVecDeque<Radical>,  // TODO implement Radical
    sigma: f64,
    iters: usize,
    montecarlo: bool,
}

enum AppMsg {
    Montecarlo(bool),
}

#[derive(relm4::Components)]
struct AppComponents {
    chart: RelmComponent<ChartModel, AppModel>
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Montecarlo(v) => {
                self.montecarlo = v;
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
            },
        }
    }

    fn post_init() {
       title
            .bind_property("title-visible", &bottom_bar, "reveal")
            .flags(gtk::glib::BindingFlags::SYNC_CREATE)
            .build();
    }
}

// -- MAIN

fn main() {
    let model = AppModel {
        empirical: None,
        theoretical: None,
        points: 1024.0,
        sweep: 100.0,
        // params: FactoryVecDeque::new(),
        sigma: 1e+20,
        iters: 0,
        montecarlo: false,
    };
    let app = RelmApp::new(model);
    app.run();
} 
