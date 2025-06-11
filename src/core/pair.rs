use core::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(
    Debug,
    Default,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
#[archive(check_bytes)]
#[archive(bound(
    serialize = "K: rkyv::Archive + rkyv::Serialize<__S>, V: rkyv::Archive + rkyv::Serialize<__S>, __S: rkyv::ser::Serializer + rkyv::ser::SharedSerializeRegistry + Sized"
))]
#[archive(bound(
    deserialize = "K: rkyv::Archive, V: rkyv::Archive, <K as rkyv::Archive>::Archived: rkyv::Deserialize<K, __D>, <V as rkyv::Archive>::Archived: rkyv::Deserialize<V, __D>, __D: rkyv::de::SharedDeserializeRegistry"
))]
pub struct Pair<K, V>
{
    pub key: K,
    pub value: V,
}

impl<K, V> Eq for Pair<K, V> where K: Ord {}

impl<K, V> PartialEq<Self> for Pair<K, V>
where
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K, V> PartialOrd<Self> for Pair<K, V>
where
    K: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K, V> Ord for Pair<K, V>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> Borrow<K> for Pair<K, V> {
    fn borrow(&self) -> &K {
        &self.key
    }
}
