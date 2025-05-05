use gtk::glib::{self, Object, subclass::prelude::*};

use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<task_row_imp::TaskRowImp>)
        @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl TaskRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub(super) fn bind(&self, task_object: &TaskObject) {
        self.imp().bind(task_object);
    }

    pub(super) fn unbind(&self) {
        self.imp().unbind();
    }
}

mod task_row_imp;
