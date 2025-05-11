use std::borrow::Cow;

use adw::subclass::prelude::*;
use gtk::glib::{self, Object, VariantTy, prelude::*};

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObjectImp>);
}

pub type IdType = u64;

impl TaskObject {
    pub fn new(task_name: &str) -> Self {
        Object::builder().property("task_name", task_name).build()
    }

    pub fn get_id(&self) -> IdType {
        self.imp().id
    }
}

impl ToVariant for TaskObject {
    fn to_variant(&self) -> glib::Variant {
        (self.checked(), self.task_name()).to_variant()
    }
}
impl FromVariant for TaskObject {
    fn from_variant(variant: &glib::Variant) -> Option<Self> {
        variant.get::<(bool, String)>().map(|(checked, task_name)| {
            let t = TaskObject::new(&task_name);
            t.set_checked(checked);
            t
        })
    }
}

impl StaticVariantType for TaskObject {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <(bool, String)>::static_variant_type()
    }
}

mod imp {
    use std::{
        cell::{Cell, RefCell},
        sync::Mutex,
    };

    use gtk::glib::{self, Properties};

    use adw::prelude::*;
    use glib::subclass::prelude::*;

    use super::IdType;

    #[derive(Properties)]
    #[properties[wrapper_type=super::TaskObject]]
    pub struct TaskObjectImp {
        #[property(get, set)]
        checked: Cell<bool>,
        #[property(get, set)]
        task_name: RefCell<String>,

        pub(super) id: IdType,
    }

    impl Default for TaskObjectImp {
        fn default() -> Self {
            static ID: Mutex<IdType> = Mutex::new(0);
            let mut id = ID.lock().unwrap();
            let self_ = Self {
                checked: Default::default(),
                task_name: Default::default(),
                id: *id,
            };

            *id += 1;

            self_
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaskObjectImp {
        const NAME: &'static str = "GtkRsTestTaskObject";
        type Type = super::TaskObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TaskObjectImp {}
}
