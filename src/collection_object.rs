use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct CollectionObject(ObjectSubclass<imp::CollectionObjectImp>);
}

pub type IdType = u64;

impl CollectionObject {
    pub fn new(title: &str) -> Self {
        Object::builder().property("title", title).build()
    }

    pub fn get_id(&self) -> IdType {
        self.imp().id
    }
}

mod imp {
    use std::{cell::RefCell, sync::Mutex};

    use gtk::{
        gio::ListStore,
        glib::{self, Properties},
    };

    use adw::prelude::*;
    use glib::subclass::prelude::*;

    use crate::task_object::TaskObject;

    use super::IdType;

    #[derive(Properties)]
    #[properties[wrapper_type=super::CollectionObject]]
    pub struct CollectionObjectImp {
        #[property(get, set)]
        title: RefCell<String>,
        #[property(get)]
        tasks: ListStore,

        pub(super) id: IdType,
    }

    impl Default for CollectionObjectImp {
        fn default() -> Self {
            static ID: Mutex<IdType> = Mutex::new(0);
            let mut id = ID.lock().unwrap();
            let self_ = Self {
                title: Default::default(),
                tasks: ListStore::new::<TaskObject>(),
                id: *id,
            };
            *id += 1;

            self_
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CollectionObjectImp {
        const NAME: &'static str = "GtkRsTestCollectionObject";
        type Type = super::CollectionObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for CollectionObjectImp {}
}
