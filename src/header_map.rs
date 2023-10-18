use std::{fmt, iter};

use http::{header::GetAll, HeaderName, HeaderValue};
use serde::{
    de,
    ser::{self, SerializeSeq},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{header_value, BorrowedNameWrapper, Either};

type Type = http::HeaderMap;
const EXPECT_MESSAGE: &str = "a header map";

#[derive(Serialize)]
struct BorrowedValueWrapper<'a>(#[serde(with = "crate::header_value")] &'a HeaderValue);

struct GetAllWrapper<'a>(GetAll<'a, HeaderValue>);

impl<'a> Serialize for GetAllWrapper<'a> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut iter = self.0.iter();
        let Some(first) = iter.next() else {
            return Err(ser::Error::custom("header has no values"));
        };

        if iter.next().is_none() {
            if ser.is_human_readable() {
                return header_value::serialize(first, ser);
            } else {
                return ser.collect_seq(iter::once(BorrowedValueWrapper(first)));
            }
        };

        let count = iter.count() + 2;
        let mut seq = ser.serialize_seq(Some(count))?;
        for v in self.0.iter() {
            seq.serialize_element(&BorrowedValueWrapper(v))?;
        }
        seq.end()
    }
}

pub fn serialize<S>(headers: &Type, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.collect_map(
        headers
            .keys()
            .map(|k| (BorrowedNameWrapper(k), GetAllWrapper(headers.get_all(k)))),
    )
}

#[derive(Deserialize)]
struct NameWrapper(#[serde(with = "crate::header_name")] HeaderName);

#[derive(Deserialize)]
struct ValueWrapper(#[serde(with = "crate::header_value")] HeaderValue);

struct Visitor {
    is_human_readable: bool,
}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(EXPECT_MESSAGE)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut map = Type::with_capacity(access.size_hint().unwrap_or(0));

        if self.is_human_readable {
            while let Some((key, val)) = access.next_entry::<NameWrapper, Either<ValueWrapper>>()? {
                match val {
                    Either::One(val) => {
                        map.insert(key.0, val.0);
                    }
                    Either::Many(arr) => {
                        for val in arr {
                            map.append(&key.0, val.0);
                        }
                    }
                };
            }
        } else {
            while let Some((key, arr)) = access.next_entry::<NameWrapper, Vec<ValueWrapper>>()? {
                for val in arr {
                    map.append(&key.0, val.0);
                }
            }
        }
        Ok(map)
    }
}

pub fn deserialize<'de, D>(de: D) -> Result<Type, D::Error>
where
    D: Deserializer<'de>,
{
    let is_human_readable = de.is_human_readable();
    de.deserialize_map(Visitor { is_human_readable })
}

derive_extension_types!(super::Type);
