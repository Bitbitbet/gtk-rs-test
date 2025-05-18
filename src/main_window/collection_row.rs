use gtk::{
    gio::{ActionEntry, SimpleActionGroup},
    glib::{self, Object, subclass::prelude::*},
};

use adw::prelude::*;

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

    fn setup_actions(&self) {
        let group = SimpleActionGroup::new();
        self.insert_action_group("row", Some(&group));

        let self0 = self.downgrade();
        group.add_action_entries([ActionEntry::builder("copy")
            .activate(move |_, _, _| {
                self0.upgrade().unwrap().imp().copy();
            })
            .build()]);
    }
}

mod collection_row_imp;
