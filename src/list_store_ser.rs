use std::marker::PhantomData;

use gtk::{
    gio::{
        ListStore,
        prelude::{ListModelExt, ListModelExtManual},
    },
    glib::{Object, prelude::IsA},
};
use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeSeq};

pub struct ListStoreSer<T>(ListStore, PhantomData<T>);

impl<T> ListStoreSer<T> {
    pub fn new(list_store: ListStore) -> Self {
        ListStoreSer {
            0: list_store,
            1: PhantomData {},
        }
    }

    pub fn extract(self) -> ListStore {
        self.0
    }
}

impl<T: Serialize + IsA<Object>> Serialize for ListStoreSer<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.n_items() as usize))?;

        for item in self.0.iter::<T>().map(Result::unwrap) {
            seq.serialize_element(&item)?;
        }

        seq.end()
    }
}

impl<'de, T: Deserialize<'de> + IsA<Object>> Deserialize<'de> for ListStoreSer<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Vis<T>(PhantomData<T>);

        impl<'de, T: IsA<Object> + Deserialize<'de>> Visitor<'de> for Vis<T> {
            type Value = ListStoreSer<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Error occurred trying to deserialize a ListStoreSer")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let list = ListStore::new::<T>();
                while let Some(ele) = seq.next_element::<T>()? {
                    list.append(&ele);
                }

                Ok(ListStoreSer {
                    0: list,
                    1: PhantomData {},
                })
            }
        }

        deserializer.deserialize_seq(Vis { 0: PhantomData {} })
    }
}
