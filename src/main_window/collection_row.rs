use gtk::glib::{self, Object, subclass::prelude::*};

use crate::collection_object::CollectionObject;

glib::wrapper! {
    pub struct CollectionRow(ObjectSubclass<collection_row_imp::CollectionRowImp>)
        @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl CollectionRow {
    pub fn new(collection_object: &CollectionObject) -> Self {
        let self_: Self = Object::builder().build();

        self_.imp().associate(collection_object);

        self_
    }
}

mod collection_row_imp;
