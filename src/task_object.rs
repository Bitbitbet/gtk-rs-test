use std::borrow::Cow;

use adw::subclass::prelude::*;
use gtk::glib::{self, Object, VariantTy, prelude::*};
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObjectImp>);
}

pub type IdType = u64;

impl TaskObject {
    pub fn new(name: &str) -> Self {
        Object::builder().property("name", name).build()
    }

    pub fn get_id(&self) -> IdType {
        self.imp().id
    }
}
impl Serialize for TaskObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.imp().serialize(serializer)
    }
}

const FIELDS: [&'static str; 2] = ["checked", "name"];
impl<'de> Deserialize<'de> for TaskObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TaskObjectVisitor;

        impl<'de> Visitor<'de> for TaskObjectVisitor {
            type Value = TaskObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Error during deserialzation of TaskObject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut checked = None;
                let mut name = None;
                for _ in 0..2 {
                    match map.next_key::<String>()?.unwrap().as_str() {
                        "checked" => {
                            checked = Some(map.next_value::<bool>()?);
                        }
                        "name" => {
                            name = Some(map.next_value::<String>()?);
                        }
                        f => return Err(de::Error::unknown_field(f, &FIELDS)),
                    }
                }
                let checked = checked.ok_or_else(|| de::Error::missing_field("checked"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("checked"))?;

                let task_object = TaskObject::new(&name);
                task_object.set_checked(checked);

                Ok(task_object)
            }
        }
        deserializer.deserialize_struct("TaskObject", &FIELDS, TaskObjectVisitor {})
    }
}
impl ToVariant for TaskObject {
    fn to_variant(&self) -> glib::Variant {
        (self.checked(), self.name()).to_variant()
    }
}
impl FromVariant for TaskObject {
    fn from_variant(variant: &glib::Variant) -> Option<Self> {
        variant.get::<(bool, String)>().map(|(checked, name)| {
            let t = TaskObject::new(&name);
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
    use serde::{Serialize, ser::SerializeStruct};

    use super::IdType;

    #[derive(Properties)]
    #[properties[wrapper_type=super::TaskObject]]
    pub struct TaskObjectImp {
        #[property(get, set)]
        checked: Cell<bool>,
        #[property(get, set)]
        name: RefCell<String>,

        pub(super) id: IdType,
    }
    impl Serialize for TaskObjectImp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut stru = serializer.serialize_struct("TaskObject", 2)?;

            stru.serialize_field("checked", &self.checked)?;
            stru.serialize_field("name", &self.name)?;

            stru.end()
        }
    }

    impl Default for TaskObjectImp {
        fn default() -> Self {
            static ID: Mutex<IdType> = Mutex::new(0);
            let mut id = ID.lock().unwrap();
            let self_ = Self {
                checked: Default::default(),
                name: Default::default(),
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
