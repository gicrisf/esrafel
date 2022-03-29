use super::*;

// Object holding the state
#[derive(Default)]
pub struct NucObject {
    eqs: Cell<i32>,
    spin_val: Cell<f32>,
    spin_var: Cell<f32>,
    hpf_val: Cell<f32>,
    hpf_var: Cell<f32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NucObject {
    const NAME: &'static str = "EsrafelGtkAppNucObject";
    type Type = super::NucObject;
    type ParentType = glib::Object;
}

// Trait that is used to override virtual methods of glib::Object.
// https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/subclass/index.html
impl ObjectImpl for NucObject {
    // Called once in the very beginning to list all properties of this class.
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecInt::new(
                    // Name
                    "eqs",
                    // Nickname
                    "eqs",
                    // Short description
                    "How many equivalents",
                    // Minimum value
                    i32::MIN,
                    // Maximum value
                    i32::MAX,
                    // Default value
                    0,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                glib::ParamSpecFloat::new(
                    "spinval",
                    "Spin_val",
                    "Spin value",
                    f32::MIN,
                    f32::MAX,
                    0.0,
                    ParamFlags::READWRITE,
                ),
                glib::ParamSpecFloat::new(
                    "spinvar",
                    "Spin_var",
                    "Spin MC variation",
                    f32::MIN,
                    f32::MAX,
                    0.0,
                    ParamFlags::READWRITE,
                ),
                glib::ParamSpecFloat::new(
                    "hpfval",
                    "Hpf_var",
                    "Hyperfine constant value",
                    f32::MIN,
                    f32::MAX,
                    0.0,
                    ParamFlags::READWRITE,
                ),
                glib::ParamSpecFloat::new(
                    "hpfvar",
                    "Hpf_var",
                    "Hyperfine constant MC variation",
                    f32::MIN,
                    f32::MAX,
                    0.0,
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "eqs" => {
                let input_number = value.get().expect("The value needs to be of type `i32`.");
                self.eqs.replace(input_number);
            },
            "spinval" => {
                let input_number = value.get().expect("The value needs to be of type `f32`.");
                self.eqs.replace(input_number);
            },
            "spinvar" => {
                let input_number = value.get().expect("The value needs to be of type `f32`.");
                self.eqs.replace(input_number);
            },
            "hpfval" => {
                let input_number = value.get().expect("The value needs to be of type `f32`.");
                self.eqs.replace(input_number);
            },
            "hpfvar" => {
                let input_number = value.get().expect("The value needs to be of type `f32`.");
                self.eqs.replace(input_number);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "number" => self.eqs.get().to_value(),
            "spinval" => self.spin_val.get().to_value(),
            "spinvar" => self.spin_var.get().to_value(),
            "hpfval" => self.hpf_val.get().to_value(),
            "hpfvar" => self.hpf_var.get().to_value(),
            _ => unimplemented!(),
        }
    }

    // Called right after construction of the instance.
    // fn constructed(&self, obj: &Self::Type) {}
}
