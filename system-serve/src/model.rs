use actix_web::HttpResponse;
use bson::oid::ObjectId;
use core::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use yn_util::*;

pub type SimpleResp = Result<HttpResponse, utils::BusinessError>;

/// 序列化 id
#[allow(dead_code)]
pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none(),
    }
}

/// 反序列化id
#[allow(dead_code)]
pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct JsonOptionObjectIdVisitor;

    impl<'de> de::Visitor<'de> for JsonOptionObjectIdVisitor {
        type Value = Option<ObjectId>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object id hash value")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                return Ok(None);
            }
            Ok(ObjectId::with_string(v).ok())
        }
    }

    deserializer.deserialize_any(JsonOptionObjectIdVisitor)
}
pub mod button;
pub mod city;
pub mod job;
pub mod menu;
pub mod org;
pub mod provinces;
pub mod request;
pub mod role;
pub mod theme;

pub mod portal_product;

pub mod user;
