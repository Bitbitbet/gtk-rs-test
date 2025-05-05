use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct CollectionObject(ObjectSubclass<imp::CollectionObjectImp>);
}

impl CollectionObject {
    pub fn new(title: &str) -> Self {
        Object::builder().property("title", title).build()
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::{
        gio::ListStore,
        glib::{self, Properties},
    };

    use adw::prelude::*;
    use glib::subclass::prelude::*;

    use crate::task_object::TaskObject;

    #[derive(Properties)]
    #[properties[wrapper_type=super::CollectionObject]]
    pub struct CollectionObjectImp {
        #[property(get, set)]
        title: RefCell<String>,
        #[property(get)]
        tasks: ListStore,
    }

    impl Default for CollectionObjectImp {
        fn default() -> Self {
            Self {
                title: Default::default(),
                tasks: ListStore::new::<TaskObject>(),
            }
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
