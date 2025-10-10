use serde::{Deserialize, Deserializer, Serialize, Serializer};
use parking_lot::RwLock;

pub fn remove_lock<S, T>(value: &RwLock<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Clone + Serialize,
{
    value.read().clone().serialize(serializer)
}

pub fn add_lock<'de, D, T>(deserializer: D) -> Result<RwLock<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = T::deserialize(deserializer)?;
    Ok(RwLock::new(value))
}