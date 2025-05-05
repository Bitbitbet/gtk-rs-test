use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObjectImp>);
}

impl TaskObject {
    pub fn new(task_name: &str) -> Self {
        Object::builder().property("task_name", task_name).build()
    }
}

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib::{self, Properties};

    use adw::prelude::*;
    use glib::subclass::prelude::*;

    #[derive(Default, Properties)]
    #[properties[wrapper_type=super::TaskObject]]
    pub struct TaskObjectImp {
        #[property(get, set)]
        checked: Cell<bool>,
        #[property(get, set)]
        task_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaskObjectImp {
        const NAME: &'static str = "GtkRsTestTaskObject";
        type Type = super::TaskObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TaskObjectImp {}
}
