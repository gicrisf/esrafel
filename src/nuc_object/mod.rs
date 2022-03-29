mod imp;

use crate::gtk::{
    subclass::prelude::*,
    glib::{self, Object, ParamFlags, ParamSpec, ParamSpecInt, ToValue, Value,},
    // prelude::{FilterExt, SorterExt},
};

use std::cell::Cell;
use once_cell::sync::Lazy;

// Optionally, define a wrapper type to make it more ergonomic to use from Rust
glib::wrapper! {
    pub struct NucObject(ObjectSubclass<imp::NucObject>);
}

impl NucObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Could not create `NucObject`.")
    }

    // You can set other functions here
    // pub fn increase_some_number(self) { ... }
}

impl Default for NucObject {
    fn default() -> Self {
        Self::new()
    }
}
