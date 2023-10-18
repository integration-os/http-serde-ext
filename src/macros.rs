macro_rules! serde_option {
    ($ty:ty$(, $generic:ident)?) => {
        pub mod option {
            #[derive(serde::Serialize)]
            struct Temp<'a$(, $generic: serde::Serialize)?>(#[serde(with = "super")] &'a $ty);

            pub fn serialize<$($generic: serde::Serialize, )?S: serde::Serializer>(
                val: &Option<$ty>,
                ser: S,
            ) -> Result<S::Ok, S::Error> {
                match *val {
                    Some(ref value) => ser.serialize_some(&Temp(value)),
                    None => ser.serialize_none(),
                }
            }

            struct Visitor$(<$generic: for<'a> serde::Deserialize<'a>>)? {
                $(ph: std::marker::PhantomData<$generic>,)?
            }

            impl<'de$(, $generic: for<'a> serde::Deserialize<'a>)?> serde::de::Visitor<'de> for Visitor$(<$generic>)? {
                type Value = Option<$ty>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("option")
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    super::deserialize(deserializer).map(Some)
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(None)
                }
            }

            pub fn deserialize<'de, D$(, $generic)?>(de: D) -> Result<Option<$ty>, D::Error>
            where
                D: serde::Deserializer<'de>,
                $($generic: for<'a> serde::Deserialize<'a>,)?
            {
                de.deserialize_option(Visitor$(::<$generic>)? {
                    $(ph: std::marker::PhantomData::<$generic>,)?
                })
            }
        }
    };
}

macro_rules! serde_seq {
    ($seq:ty, $ty:ty, $create:expr, $insert:ident, $name:ident$(, $generic:ident)?) => {
        pub mod $name {
            #[derive(serde::Deserialize)]
            struct TempDe$(<$generic: for<'a> serde::Deserialize<'a>>)?(#[serde(with = "super")] $ty);

            #[derive(serde::Serialize)]
            struct TempSer<'a$(, $generic: serde::Serialize)?>(#[serde(with = "super")] &'a $ty);


            pub fn serialize<$($generic: serde::Serialize, )?S: serde::Serializer>(
                val: &$seq,
                ser: S,
            ) -> Result<S::Ok, S::Error> {

                let mut seq = ser.serialize_seq(Some(val.len()))?;
                for val in val {
                    serde::ser::SerializeSeq::serialize_element(&mut seq, &TempSer(val))?;
                }
                serde::ser::SerializeSeq::end(seq)
            }

            struct Visitor$(<$generic: for<'a> serde::Deserialize<'a>>)? {
                $(ph: std::marker::PhantomData<$generic>,)?
            }

            impl<'de$(, $generic: for<'a> serde::Deserialize<'a>)?> serde::de::Visitor<'de> for Visitor$(<$generic>)? {
                type Value = $seq;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("option")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                where
                    V: serde::de::SeqAccess<'de>,
                {
                    #[allow(clippy::redundant_closure_call)]
                    let mut ret = $create(seq.size_hint().unwrap_or_default());
                    while let Some(val) = seq.next_element::<TempDe$(<$generic>)?>()? {
                        ret.$insert(val.0);
                    }
                    Ok(ret)
                }
            }

            pub fn deserialize<'de, D$(, $generic)?>(de: D) -> Result<$seq, D::Error>
            where
                D: serde::Deserializer<'de>,
                $($generic: for<'a> serde::Deserialize<'a>,)?
            {
                de.deserialize_seq(Visitor$(::<$generic>)? {
                    $(ph: std::marker::PhantomData::<$generic>,)?
                })
            }
        }
    };
}

macro_rules! serde_map {
    ($map:ty, $key:ident, $ty:ty, $create:expr, $insert:ident, $name:ident$(, $generic:ident)?) => {
        pub mod $name {

            #[derive(serde::Deserialize)]
            struct TempDe$(<$generic: for<'a> serde::Deserialize<'a>>)?(#[serde(with = "super")] $ty);

            #[derive(serde::Serialize)]
            struct TempSer<'a$(, $generic: serde::Serialize)?>(#[serde(with = "super")] &'a $ty);

            pub fn serialize<$($generic: serde::Serialize, )?$key: serde::Serialize, S: serde::Serializer>(
                val: &$map,
                ser: S,
            ) -> Result<S::Ok, S::Error> {

                let mut map = ser.serialize_map(Some(val.len()))?;
                for (k, val) in val {
                    serde::ser::SerializeMap::serialize_entry(&mut map, k, &TempSer(val))?;
                }
                serde::ser::SerializeMap::end(map)
            }

            struct Visitor<$key: for<'a> serde::Deserialize<'a>$(, $generic: for<'a> serde::Deserialize<'a>)?> {
                ph_k: std::marker::PhantomData<$key>,
                $(ph: std::marker::PhantomData<$generic>,)?
            }

            impl<'de$(, $generic: for<'a> serde::Deserialize<'a>)?, $key: for<'a> serde::Deserialize<'a> + std::cmp::Ord + std::cmp::Eq + std::cmp::PartialEq + std::hash::Hash> serde::de::Visitor<'de> for Visitor<$key, $($generic)?> {
                type Value = $map;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("option")
                }

                fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {

                    #[allow(clippy::redundant_closure_call)]
                    let mut ret = $create(map.size_hint().unwrap_or_default());
                    while let Some((k, val)) = map.next_entry::<$key, TempDe$(<$generic>)?>()? {
                        ret.$insert(k, val.0);
                    }
                    Ok(ret)
                }
            }

            pub fn deserialize<'de, D$(, $generic)?, $key: for<'a> serde::Deserialize<'a> + std::cmp::Ord + std::cmp::Eq + std::cmp::PartialEq + std::hash::Hash>(de: D) -> Result<$map, D::Error>
            where
                D: serde::Deserializer<'de>,
                $($generic: for<'a> serde::Deserialize<'a>,)?
            {
                de.deserialize_map(Visitor$(::<$key, $generic>)? {
                    ph_k: std::marker::PhantomData::<$key>,
                    $(ph: std::marker::PhantomData::<$generic>,)?
                })
            }
        }
    };
}

macro_rules! derive_extension_types {
    ($ty:ty$(, $generic:ident)?) => {
        serde_option!($ty$(, $generic)?);
        serde_seq!(Vec<$ty>, $ty, Vec::with_capacity, push, vec$(, $generic)?);
        serde_seq!(
            std::collections::VecDeque<$ty>,
            $ty,
            std::collections::VecDeque::with_capacity,
            push_back,
            vec_deque$(, $generic)?
        );
        serde_seq!(
            std::collections::LinkedList<$ty>,
            $ty,
            |_| std::collections::LinkedList::new(),
            push_back,
            linked_list$(, $generic)?
        );
        serde_map!(
            std::collections::HashMap<K, $ty>,
            K,
            $ty,
            std::collections::HashMap::with_capacity,
            insert,
            hash_map$(, $generic)?
        );
        serde_map!(
            std::collections::BTreeMap<K, $ty>,
            K,
            $ty,
            |_| std::collections::BTreeMap::new(),
            insert,
            btree_map$(, $generic)?
        );
    }
}

macro_rules! serialize_as_str {
    ($ty:ty) => {
        pub fn serialize<S: serde::Serializer>(val: &$ty, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(&val.as_str())
        }
    };
}

macro_rules! create_visitor {
    ($visitor:ident, $ty:ty, $msg:tt, $(($visit_name:ident, $visit_type:ty)),+) => {
        struct $visitor;

        impl<'de> serde::de::Visitor<'de> for $visitor {
            type Value = $ty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str($msg)
            }

            $(fn $visit_name<E: serde::de::Error>(self, val: $visit_type) -> Result<Self::Value, E> {
                val.try_into().map_err(serde::de::Error::custom)
            })+
        }
    }
}

macro_rules! deserialize_from_str {
    ($visitor:ident, $ty:ty) => {
        pub fn deserialize<'de, D>(de: D) -> Result<$ty, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            de.deserialize_str($visitor)
        }
    };
}

macro_rules! deserialize_string {
    ($ty:ty, $visitor:ident) => {
        pub fn deserialize<'de, D>(de: D) -> Result<$ty, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            de.deserialize_string($visitor)
        }
    };
}

macro_rules! serde_request_response {
    ($ty:ty, $name:tt, $head:ty, $borrowed_head:ty) => {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Head,
            Body,
        }

        impl Field {
            const fn as_str(&self) -> &'static str {
                match self {
                    Field::Head => "head",
                    Field::Body => "body",
                }
            }

            const fn len() -> usize {
                2
            }
        }

        pub fn serialize<S, T>(val: &$ty, ser: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
            T: serde::Serialize,
        {
            if !val.extensions().is_empty() {
                return Err(serde::ser::Error::custom("extensions is not empty"));
            }
            let mut state = ser.serialize_struct(STRUCT_NAME, Field::len())?;
            serde::ser::SerializeStruct::serialize_field(
                &mut state,
                Field::Head.as_str(),
                &<$borrowed_head>::from(val),
            )?;
            serde::ser::SerializeStruct::serialize_field(
                &mut state,
                Field::Body.as_str(),
                val.body(),
            )?;
            serde::ser::SerializeStruct::end(state)
        }

        struct Visitor<T> {
            ph: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: for<'a> serde::Deserialize<'a>,
        {
            type Value = $ty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str($name)
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let head: $head = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let body = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                head.try_into(body)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut head: Option<$head> = None;
                let mut body = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Head => {
                            if head.is_some() {
                                return Err(serde::de::Error::duplicate_field(key.as_str()));
                            }
                            head = Some(map.next_value()?);
                        }
                        Field::Body => {
                            if body.is_some() {
                                return Err(serde::de::Error::duplicate_field(key.as_str()));
                            }
                            body = Some(map.next_value()?);
                        }
                    }
                }
                let head =
                    head.ok_or_else(|| serde::de::Error::missing_field(Field::Head.as_str()))?;
                let body =
                    body.ok_or_else(|| serde::de::Error::missing_field(Field::Body.as_str()))?;

                head.try_into(body)
            }
        }

        pub fn deserialize<'de, T, D>(de: D) -> Result<$ty, D::Error>
        where
            T: for<'a> serde::Deserialize<'a>,
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[Field::Head.as_str(), Field::Body.as_str()];
            de.deserialize_struct(
                $name,
                FIELDS,
                Visitor::<T> {
                    ph: std::marker::PhantomData,
                },
            )
        }
    };
}
