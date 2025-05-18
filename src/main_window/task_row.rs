use adw::subclass::prelude::*;
use gtk::{
    gio::{ActionEntry, SimpleActionGroup, prelude::ActionMapExtManual},
    glib::{self, Object, clone::Downgrade},
    prelude::WidgetExt,
};

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

    pub(super) fn bind(&self, task_object: &TaskObject) {
        self.imp().bind(task_object);
    }

    pub(super) fn unbind(&self) {
        self.imp().unbind();
    }
}

mod task_row_imp;
