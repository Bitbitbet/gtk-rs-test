use std::cell::RefCell;

use adw::ActionRow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CheckButton;
use gtk::CompositeTemplate;
use gtk::GestureClick;
use gtk::PopoverMenu;
use gtk::gdk::BUTTON_SECONDARY;
use gtk::gdk::Rectangle;
use gtk::gio::Menu;
use gtk::gio::MenuItem;
use gtk::glib;
use gtk::glib::Binding;
use gtk::glib::Properties;
use gtk::glib::object::CastNone;
use gtk::glib::object::ObjectExt;
use gtk::glib::subclass::InitializingObject;

use crate::task_object::TaskObject;

#[derive(CompositeTemplate, Properties)]
#[properties(wrapper_type = super::TaskRow)]
#[template(resource = "/me/gtk-rs-test/test/task_row.ui")]
pub struct TaskRowImp {
    #[template_child]
    check_button: TemplateChild<CheckButton>,
    #[template_child]
    rightclick_menu: TemplateChild<PopoverMenu>,
    #[template_child]
    rightclick_menu_model: TemplateChild<Menu>,

    #[property(get, set)]
    title: RefCell<String>,

    task_name: RefCell<String>,
    bindings: RefCell<Vec<Binding>>,
    delete_menu_item: MenuItem,
}

impl Default for TaskRowImp {
    fn default() -> Self {
        Self {
            check_button: Default::default(),
            rightclick_menu: Default::default(),
            rightclick_menu_model: Default::default(),
            title: Default::default(),
            task_name: Default::default(),
            bindings: Default::default(),
            delete_menu_item: MenuItem::new(Some("Delete"), None),
        }
    }
}

impl TaskRowImp {
    pub fn bind(&self, task_object: &TaskObject) {
        let mut bindings = self.bindings.borrow_mut();

        self.update_menu_item(task_object);

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

    fn update_menu_item(self: &Self, task_object: &TaskObject) {
        self.rightclick_menu_model.remove_all();
        self.delete_menu_item.set_action_and_target_value(
            Some("win.remove-task"),
            Some(&task_object.get_id().to_variant()),
        );
        self.rightclick_menu_model
            .append_item(&self.delete_menu_item);
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

        self.rightclick_menu.set_parent(&*self.obj());

        let gesture_click = GestureClick::builder().button(BUTTON_SECONDARY).build();
        {
            let popover_menu = self.rightclick_menu.clone();
            gesture_click.connect_pressed(move |_, _, x, y| {
                popover_menu.set_pointing_to(Some(&Rectangle::new(x as i32, y as i32, 1, 1)));
                popover_menu.popup();
            });
        }
        self.obj().add_controller(gesture_click);
    }
    fn dispose(&self) {
        self.rightclick_menu.unparent();
    }
}
impl WidgetImpl for TaskRowImp {}

impl ListBoxRowImpl for TaskRowImp {}
impl PreferencesRowImpl for TaskRowImp {}
impl ActionRowImpl for TaskRowImp {}
