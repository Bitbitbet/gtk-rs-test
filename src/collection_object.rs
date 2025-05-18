use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::prelude::ListModelExtManual,
    glib::{self, Object},
};
use gtk_rs_test::list_store_ser::ListStoreSer;
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use crate::task_object::TaskObject;

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

impl Serialize for CollectionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.imp().serialize(serializer)
    }
}

const FIELDS: [&'static str; 2] = ["title", "tasks"];
impl<'de> Deserialize<'de> for CollectionObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CollectionObjectVisitor;

        impl<'de> Visitor<'de> for CollectionObjectVisitor {
            type Value = CollectionObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expecting a serialized CollecitonObject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut title = None;
                let mut tasks = None;
                for _ in 0..2 {
                    match map.next_key::<String>()?.unwrap().as_str() {
                        "title" => {
                            title = Some(map.next_value::<String>()?);
                        }
                        "tasks" => {
                            tasks = Some(map.next_value::<ListStoreSer<TaskObject>>()?);
                        }
                        f => return Err(de::Error::unknown_field(f, &FIELDS)),
                    }
                }
                let title = title.ok_or_else(|| de::Error::missing_field("checked"))?;
                let tasks = tasks
                    .ok_or_else(|| de::Error::missing_field("checked"))?
                    .extract();

                let collection_object = CollectionObject::new(&title);
                let tasks_dest = collection_object.tasks();
                for t in tasks.iter::<TaskObject>().map(Result::unwrap) {
                    tasks_dest.append(&t);
                }
                Ok(collection_object)
            }
        }
        deserializer.deserialize_struct("CollectionObject", &FIELDS, CollectionObjectVisitor {})
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
    use gtk_rs_test::list_store_ser::ListStoreSer;
    use serde::{Serialize, Serializer, ser::SerializeStruct};

    use crate::task_object::TaskObject;

    use super::IdType;

    #[derive(Properties)]
    #[properties[wrapper_type=super::CollectionObject]]
    pub struct CollectionObjectImp {
        #[property(get, set)]
        title: RefCell<String>,

        /// ListStore storing TaskObject
        #[property(get)]
        tasks: ListStore,

        pub(super) id: IdType,
    }
    impl Serialize for CollectionObjectImp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut stru = serializer.serialize_struct("CollectionObject", 2)?;
            stru.serialize_field("title", &self.title)?;
            stru.serialize_field(
                "tasks",
                &ListStoreSer::<TaskObject>::new(self.tasks.clone()),
            )?;
            stru.end()
        }
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
