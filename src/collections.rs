pub use cc_traits::{
	// Immutable traits
	Collection, 
    Keyed,
    KeyedRef,
    SimpleKeyedRef,
    Len, 
    CollectionRef,
    SimpleCollectionRef,
    Get,
    GetKeyValue,
    Map, 

    // Mutable traits
    CollectionMut,
    SimpleCollectionMut,
    GetMut,
    MapInsert,
    Remove,
    Clear,
	MapMut,

    // Iterators
    Iter,
	IterMut,
	MapIter,
	MapIterMut,
};

use foldhash::{
    HashMap,
    HashMapExt,
};
use std::{
    hash::Hash,
    ops::{ Deref, DerefMut },
};

pub struct FastMap<K, V> {
    base: HashMap<K, V>,
}

impl<K, V> FastMap<K, V> 
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self { base: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { base: HashMap::with_capacity(capacity) }
    }
}

impl<K, V> Deref for FastMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<K, V> DerefMut for FastMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<K, V> FromIterator<(K, V)> for FastMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> FastMap<K, V> {
        Self { base: HashMap::from_iter(iter.into_iter()) }
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for FastMap<K, V>
where
    K: Hash + Eq,
{
    fn from(arr: [(K, V); N]) -> Self {
        Self { base: HashMap::from_iter(arr.into_iter()) }
    }
}

impl<K, V> Collection for FastMap<K, V> {
    type Item = V;
}

impl<K, V> Keyed for FastMap<K, V> {
    type Key = K;
}

impl<K, V> Len for FastMap<K, V> 
{
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> CollectionRef for FastMap<K, V> {
    type ItemRef<'a> = &'a V where Self: 'a;

    cc_traits::covariant_item_ref!();
}

impl<K, V> SimpleCollectionRef for FastMap<K, V> {
    cc_traits::simple_collection_ref!();
}

impl<K, V> KeyedRef for FastMap<K, V> {
    type KeyRef<'a> = &'a K where Self: 'a;

    cc_traits::covariant_key_ref!();
}

impl<K, V> SimpleKeyedRef for FastMap<K, V> {
    cc_traits::simple_keyed_ref!();
}

impl<K, V> Get<&K> for FastMap<K, V> 
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<Self::ItemRef<'_>> {
        self.base.get(key)
    }
}

impl<K, V> GetKeyValue<&K> for FastMap<K, V> 
where
    K: Hash + Eq,
{
    fn get_key_value(&self, key: &K) -> Option<(Self::KeyRef<'_>, Self::ItemRef<'_>)> {
        self.base.get_key_value(key)
    }
}

impl<K, V> CollectionMut for FastMap<K, V> {
    type ItemMut<'a> = &'a mut V where Self: 'a;

    cc_traits::covariant_item_mut!();
}

impl<K, V> SimpleCollectionMut for FastMap<K, V> {
    cc_traits::simple_collection_mut!();
}

impl<K, V> GetMut<&K> for FastMap<K, V> 
where
    K: Hash + Eq,
{
    fn get_mut(&mut self, key: &K) -> Option<Self::ItemMut<'_>> {
        self.base.get_mut(key)
    }
}

impl<K, V> MapInsert<K> for FastMap<K, V> 
where
    K: Hash + Eq,
{
    type Output = Option<V>;

    fn insert(&mut self, key: K, value: Self::Item) -> Self::Output {
        self.base.insert(key, value)
    }
}

impl<K, V> Remove<&K> for FastMap<K, V> 
where
    K: Hash + Eq,
{
    fn remove(&mut self, key: &K) -> Option<Self::Item> {
        self.base.remove(key)
    }
}

impl<K, V> Clear for FastMap<K, V> {
    fn clear(&mut self) {
        self.base.clear();
    }
}

impl<K, V> Iter for FastMap<K, V>
{
    type Iter<'a> = std::collections::hash_map::Values<'a, K, V> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.base.values()
    }
}

impl<K, V> IterMut for FastMap<K, V>
{
    type IterMut<'a> = std::collections::hash_map::ValuesMut<'a, K, V> where Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.base.values_mut()
    }
}

impl<K, V> MapIter for FastMap<K, V> {
    type Iter<'a> = std::collections::hash_map::Iter<'a, K, V> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.base.iter()
    }
}

impl<K, V> MapIterMut for FastMap<K, V> {
    type IterMut<'a> = std::collections::hash_map::IterMut<'a, K, V> where Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.base.iter_mut()
    }
}