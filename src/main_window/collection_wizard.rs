use adw::Dialog;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct CollectionWizard(ObjectSubclass<collection_wizard_imp::CollectionWizardImp>)
        @extends Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl CollectionWizard {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod collection_wizard_imp;
