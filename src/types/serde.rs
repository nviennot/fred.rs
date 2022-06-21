use crate::types::{RedisKey, RedisMap, RedisValue};
use nom::AsBytes;
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Formatter};

#[cfg_attr(docsrs, doc(cfg(feature = "serde-support")))]
impl Serialize for RedisKey {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_bytes(self.key.as_bytes())
  }
}

struct RedisKeyVisitor;

impl<'de> Visitor<'de> for RedisKeyVisitor {
  type Value = RedisKey;

  fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
    todo!()
  }

  fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }

  fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
  where
    E: Error,
  {
    todo!()
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde-support")))]
impl<'de> Deserialize<'de> for RedisKey {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    todo!()
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde-support")))]
impl Serialize for RedisMap {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    todo!()
  }
}

struct RedisMapVisitor;

impl<'de> Visitor<'de> for RedisMapVisitor {
  type Value = RedisKey;

  fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
    todo!()
  }

  fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
  where
    A: MapAccess<'de>,
  {
    todo!()
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: SeqAccess<'de>,
  {
    todo!()
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde-support")))]
impl<'de> Deserialize<'de> for RedisMap {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    todo!()
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde-support")))]
impl Serialize for RedisValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    todo!()
  }
}

struct RedisValueVisitor;

impl<'de> Visitor<'de> for RedisValueVisitor {
  type Value = RedisKey;

  fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
    todo!()
  }

  fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
  where
    A: MapAccess<'de>,
  {
    todo!()
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: SeqAccess<'de>,
  {
    todo!()
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde-support")))]
impl<'de> Deserialize<'de> for RedisValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    todo!()
  }
}
