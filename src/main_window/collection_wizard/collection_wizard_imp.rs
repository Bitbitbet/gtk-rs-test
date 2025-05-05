use adw::prelude::AdwDialogExt;
use adw::{Dialog, subclass::prelude::*};
use glib::object::ObjectExt;
use glib::subclass::InitializingObject;
use glib::variant::ToVariant;
use gtk::Button;
use gtk::CompositeTemplate;
use gtk::Entry;
use gtk::glib;
use gtk::prelude::{EditableExt, WidgetExt};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/gtk-rs-test/test/collection_wizard.ui")]
pub struct CollectionWizardImp {
    #[template_child]
    create_button: TemplateChild<Button>,
    #[template_child]
    entry: TemplateChild<Entry>,
}

#[gtk::template_callbacks]
impl CollectionWizardImp {
    #[template_callback]
    fn handle_create_button_clicked(&self) {
        self.done();
    }
    #[template_callback]
    fn handle_cancel_button_clicked(&self) {
        self.cancel();
    }
    #[template_callback]
    fn handle_entry_activated(&self) {
        self.done();
    }

    fn done(&self) {
        let title = self.entry.text();
        let title = title.trim();
        if title.len() == 0 {
            return;
        }

        self.obj()
            .activate_action("win.add-collection", Some(&title.to_variant()))
            .unwrap();

        self.obj().close();
    }
    fn cancel(&self) {
        self.obj().close();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionWizardImp {
    const NAME: &'static str = "GtkRsTestCollectionWizard";
    type Type = super::CollectionWizard;
    type ParentType = Dialog;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CollectionWizardImp {
    fn constructed(&self) {
        self.parent_constructed();

        self.entry
            .bind_property("text", &*self.create_button, "sensitive")
            .transform_to(|_, text: &str| Some(text.trim().len() != 0))
            .sync_create()
            .build();
        self.entry
            .bind_property("text", &*self.entry, "css-classes")
            .transform_to(|_, text: &str| {
                Some(if text.trim().len() == 0 {
                    &["error"][..]
                } else {
                    &[][..]
                })
            })
            .build();
    }
}
impl WidgetImpl for CollectionWizardImp {}
impl AdwDialogImpl for CollectionWizardImp {}
