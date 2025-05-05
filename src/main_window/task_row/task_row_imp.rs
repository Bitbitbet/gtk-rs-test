use std::cell::RefCell;

use adw::ActionRow;
use adw::subclass::prelude::*;
use gtk::CheckButton;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::glib::Binding;
use gtk::glib::Properties;
use gtk::glib::object::CastNone;
use gtk::glib::object::ObjectExt;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::CheckButtonExt;

use crate::task_object::TaskObject;

#[derive(CompositeTemplate, Properties, Default)]
#[properties(wrapper_type = super::TaskRow)]
#[template(resource = "/me/gtk-rs-test/test/task_row.ui")]
pub struct TaskRowImp {
    #[template_child]
    check_button: TemplateChild<CheckButton>,

    #[property(get, set)]
    title: RefCell<String>,

    task_name: RefCell<String>,
    bindings: RefCell<Vec<Binding>>,
}

impl TaskRowImp {
    pub fn bind(&self, task_object: &TaskObject) {
        let mut bindings = self.bindings.borrow_mut();
        bindings.push(
            task_object
                .bind_property("checked", &*self.check_button, "active")
                .sync_create()
                .bidirectional()
                .build(),
        );
        bindings.push(
            task_object
                .bind_property("task_name", &*self.obj(), "title")
                .transform_to(|binding, task_name: &str| {
                    let self_ = binding.target().and_downcast::<super::TaskRow>().unwrap();
                    let self_ = self_.imp();

                    *self_.task_name.borrow_mut() = task_name.to_string();

                    Some(self_.title(None, Some(task_name)))
                })
                .sync_create()
                .build(),
        );
    }
    pub fn unbind(&self) {
        self.bindings
            .borrow_mut()
            .drain(..)
            .for_each(|b| b.unbind());
    }

    fn title(&self, active: Option<bool>, task_name: Option<&str>) -> String {
        let active = active.unwrap_or(self.check_button.is_active());
        let task_name_from_self = self.task_name.borrow();
        let task_name = task_name.unwrap_or(&*task_name_from_self);

        if active {
            format!("<span strikethrough=\"true\">{}</span>", task_name)
        } else {
            task_name.to_string()
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TaskRowImp {
    const NAME: &'static str = "GtkRsTestTaskRow";
    type Type = super::TaskRow;
    type ParentType = ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TaskRowImp {
    fn constructed(&self) {
        self.parent_constructed();

        self.check_button
            .bind_property("active", &*self.obj(), "title")
            .transform_to(|binding, active: bool| {
                let self_ = binding.target().and_downcast::<super::TaskRow>().unwrap();
                let self_ = self_.imp();

                Some(self_.title(Some(active), None))
            })
            .build();
    }
}
impl WidgetImpl for TaskRowImp {}

impl ListBoxRowImpl for TaskRowImp {}
impl PreferencesRowImpl for TaskRowImp {}
impl ActionRowImpl for TaskRowImp {}
