//! Defines the implementation of our model

use crate::gtk;
use gio::subclass::prelude::*;
use gtk::{gio, glib, prelude::*};

use std::cell::RefCell;

use crate::row_data::RowData;

#[derive(Debug, Default)]
pub struct NucModel(pub(super) RefCell<Vec<RowData>>);

/// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for NucModel {
    const NAME: &'static str = "NucModel";
    type Type = super::NucModel;
    type Interfaces = (gio::ListModel,);
}

impl ObjectImpl for NucModel {}

impl ListModelImpl for NucModel {
    fn item_type(&self, _list_model: &Self::Type) -> glib::Type {
        RowData::static_type()
    }
    fn n_items(&self, _list_model: &Self::Type) -> u32 {
        self.0.borrow().len() as u32
    }
    fn item(&self, _list_model: &Self::Type, position: u32) -> Option<glib::Object> {
        self.0
            .borrow()
            .get(position as usize)
            .map(|o| o.clone().upcast::<glib::Object>())
    }
}
