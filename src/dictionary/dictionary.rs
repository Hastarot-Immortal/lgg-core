use crate::{
    Word,
    collections::FastMap,
};

use std::{
    hash::Hash,
    marker::PhantomData
};

use cc_traits::{ 
    Collection, 
    Keyed,
    KeyedRef,
    SimpleKeyedRef,
    Len, 
    CollectionRef,
    Get,
    GetKeyValue,
    SimpleCollectionRef,

    CollectionMut,
    SimpleCollectionMut,
    GetMut,
    MapInsert,
    Remove,
    Clear,

    MapIter,
    MapIterMut,
};

/// A lookup collection mapping unique keys of type `I` to programmatic [`Word`] tokens.
///
/// # Type Parameters
/// * `I`: The identifier type used as a lookup key (e.g., an integer ID, or a string slice).
/// * `M`: The underlying collection map type. Defaults to [`FastMap<I, Word>`].
/// 
/// ```
/// use lgg_core::{Dictionary, Word, PartOfSpeech, Sound};
///
/// let mut dict = Dictionary::new();
/// let word = Word::from_slice(&[Sound::vowel('a')], PartOfSpeech::Noun);
/// 
/// dict.insert("apple".to_string(), word.clone());
/// 
/// assert_eq!(dict.get(&"apple".to_string()), Some(&word));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary<I, M = FastMap<I, Word>> {
    pub(crate) words: M,
    _id_type: PhantomData<I>,
}

impl<I> Dictionary<I>
where
    I: Hash + Eq,
{
    /// Instantiates an empty `Dictionary`.
    ///
    /// ```
    /// use lgg_core::Dictionary;
    ///
    /// let dict: Dictionary<u32> = Dictionary::new();
    /// assert!(dict.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { 
            words: FastMap::new(), 
            _id_type: PhantomData
        }
    }

    /// Instantiates an empty `Dictionary` pre-allocated to store a specific quantity of elements.
    ///
    /// ```
    /// use lgg_core::Dictionary;
    ///
    /// let dict: Dictionary<u32> = Dictionary::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            words: FastMap::with_capacity(capacity), 
            _id_type: PhantomData
        }
    }

    /// Instantiates a `Dictionary` from a fixed-size array of key-value tuples.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let word = Word::from_array([], PartOfSpeech::Noun);
    /// let dict = Dictionary::from_array([(1u32, word)]);
    /// 
    /// assert_eq!(dict.len(), 1);
    /// ```
    pub fn from_array<const N: usize>(arr: [(I, Word); N]) -> Self {
        <Self as From<[(I, Word); N]>>::from(arr)
    }

    /// Instantiates a `Dictionary` from an owned vector of key-value tuples.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let word = Word::from_array([], PartOfSpeech::Noun);
    /// let dict = Dictionary::from_vec(vec![(1u32, word)]);
    /// 
    /// assert_eq!(dict.len(), 1);
    /// ```
    pub fn from_vec(vec: Vec<(I, Word)>) -> Self {
        Self {
            words: FastMap::from_iter(vec.into_iter()),
            _id_type: PhantomData
        }
    }
}

impl<I, M> Dictionary<I, M> {
    /// Returns the total number of words inside the dictionary.
    ///
    /// ```
    /// use lgg_core::Dictionary;
    ///
    /// let dict: Dictionary<u32> = Dictionary::new();
    /// assert_eq!(dict.len(), 0);
    /// ```
    pub fn len(&self) -> usize 
    where
        M: Len
    {
        Len::len(self)
    }

    /// Returns `true` if the dictionary contains no elements.
    ///
    /// ```
    /// use lgg_core::Dictionary;
    ///
    /// let dict: Dictionary<u32> = Dictionary::new();
    /// assert!(dict.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool
    where
        M: Len
    {
        self.len() == 0
    }

    /// Returns a shared reference to the [`Word`] matching the provided key, or [`None`] if not found.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert(42, Word::from_array([], PartOfSpeech::Verb));
    /// 
    /// assert!(dict.get(&42).is_some());
    /// assert!(dict.get(&99).is_none());
    /// ```
    pub fn get(&self, key: &I) -> Option<&Word> 
    where
        M: for<'a> Get<&'a I>,
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
    {
        Get::get(self, key)
    }

    /// Checks if a word matching the target key exists in the collection.
    ///
    /// ```
    /// use lgg_core::Dictionary;
    ///
    /// let mut dict: Dictionary<u32> = Dictionary::new();
    /// assert!(!dict.contains(&1));
    /// ```
    pub fn contains(&self, key: &I) -> bool
    where
        M: for<'a> Get<&'a I>,
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
    {
        Get::contains(self, key)
    }

    /// Returns a reference tuple matching both the key and its assigned [`Word`], or [`None`] if missing.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert(10, Word::from_array([], PartOfSpeech::Adj));
    /// 
    /// if let Some((key, word)) = dict.get_key_value(&10) {
    ///     assert_eq!(*key, 10);
    /// }
    /// ```
    pub fn get_key_value(&self, key: &I) -> Option<(&I, &Word)>
    where
        M: for<'a> GetKeyValue<&'a I>, 
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
        for<'a> <M as KeyedRef>::KeyRef<'a>: Into<&'a I>,
    {
        GetKeyValue::get_key_value(self, key)
    }

    /// Returns a mutable reference to the [`Word`] matching the provided key, or [`None`] if not found.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech, Sound};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert(1, Word::from_array([], PartOfSpeech::Noun));
    /// 
    /// if let Some(word) = dict.get_mut(&1) {
    ///     word.push(Sound::vowel('a'));
    /// }
    /// ```
    pub fn get_mut(&mut self, key: &I) -> Option<&mut Word>
    where
        M: for<'a> GetMut<&'a I>,
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
        for<'a> <M as CollectionMut>::ItemMut<'a>: Into<&'a mut Word>,
    {
        GetMut::get_mut(self, key)
    }

    /// Inserts a key-value entry into the dictionary. 
    ///
    /// If the key already exists, the entry is overwritten and its old [`Word`] payload is returned.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let mut dict = Dictionary::new();
    /// let word = Word::from_array([], PartOfSpeech::Noun);
    /// 
    /// assert!(dict.insert(1, word).is_none());
    /// ```
    pub fn insert(&mut self, key: I, value: Word) -> Option<Word> 
    where
        M: MapInsert<I, Output = Option<Word>>,
        <M as Collection>::Item: From<Word>,
    {
        MapInsert::insert(self, key, value)
    }

    /// Removes a key-value entry from the dictionary using its identifier, returning the matched [`Word`] if it existed.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert(1, Word::from_array([], PartOfSpeech::Noun));
    /// 
    /// assert!(dict.remove(&1).is_some());
    /// assert!(dict.get(&1).is_none());
    /// ```
    pub fn remove(&mut self, key: &I) -> Option<Word> 
    where
        M: for<'a> Remove<&'a I>,
        <M as Collection>::Item: Into<Word>,
    {
        Remove::remove(self, key)
    }

    /// Clears the dictionary, removing all key-value pairs.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert(1, Word::from_array([], PartOfSpeech::Noun));
    /// dict.clear();
    /// 
    /// assert!(dict.is_empty());
    /// ```
    pub fn clear(&mut self)
    where
        M: Clear
    {
        Clear::clear(self)
    }

    /// Creates an iterator visiting all key-value entries in arbitrary order. 
    ///
    /// Yields references of type `(&I, &Word)`.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert("test", Word::from_array([], PartOfSpeech::Noun));
    /// 
    /// for (key, word) in dict.iter() {
    ///     println!("{}: {:?}", key, word);
    /// }
    /// ```
    pub fn iter(&self) -> crate::DictIter<'_, I, M>
    where
        M: MapIter,
        for<'a> M::KeyRef<'a>: Into<&'a I>,
        for<'a> M::ItemRef<'a>: Into<&'a Word>,
    {
        MapIter::iter(self)
    }

    /// Creates an iterator visiting all key-value entries mutably in arbitrary order.
    ///
    /// Yields references of type `(&I, &mut Word)`.
    ///
    /// ```
    /// use lgg_core::{Dictionary, Word, PartOfSpeech, Sound};
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert(1, Word::from_array([], PartOfSpeech::Noun));
    /// 
    /// for (id, word) in dict.iter_mut() {
    ///     word.push(Sound::vowel('u'));
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> crate::DictIterMut<'_, I, M>
    where
        M: MapIterMut,
        for<'a> M::KeyRef<'a>: Into<&'a I>,
        for<'a> M::ItemMut<'a>: Into<&'a mut Word>,
    {
        MapIterMut::iter_mut(self)
    }
}

impl<I, M, const N: usize> From<[(I, Word); N]> for Dictionary<I, M>
where
    M: From<[(I, Word); N]>,
{
    fn from(arr: [(I, Word); N]) -> Self {
        Self {
            words: M::from(arr),
            _id_type: PhantomData,
        }
    }
}

impl<I, M> Default for Dictionary<I, M>
where
    M: Default
{
    fn default() -> Self {
        Self { 
            words: M::default(), 
            _id_type: PhantomData 
        }
    }
}

impl<I, M> FromIterator<(I, Word)> for Dictionary<I, M>
where
    M: FromIterator<(I, Word)>,
{
    fn from_iter<T: IntoIterator<Item = (I, Word)>>(iter: T) -> Dictionary<I, M> {
        Self {
            words: M::from_iter(iter.into_iter()),
            _id_type: PhantomData
        }
    }
}

impl<I, M> Collection for Dictionary<I, M> {
    type Item = Word;
}

impl<I, M> Keyed for Dictionary<I, M> {
    type Key = I;
}

impl<I, M> Len for Dictionary<I, M> 
where
    M: Len
{
    fn len(&self) -> usize {
        self.words.len()
    }
}

impl<I, M> CollectionRef for Dictionary<I, M> {
    type ItemRef<'a> = &'a Word where Self: 'a;

    cc_traits::covariant_item_ref!();
}

impl<I, M> SimpleCollectionRef for Dictionary<I, M> {
    cc_traits::simple_collection_ref!();
}

impl<I, M> KeyedRef for Dictionary<I, M> {
    type KeyRef<'a> = &'a I where Self: 'a;

    cc_traits::covariant_key_ref!();
}

impl<I, M> SimpleKeyedRef for Dictionary<I, M> {
    cc_traits::simple_keyed_ref!();
}

impl<I, M> Get<&I> for Dictionary<I, M>
where
    M: for<'a> Get<&'a I>,
    for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
{
    fn get(&self, key: &I) -> Option<Self::ItemRef<'_>> {
        self.words.get(key).map(Into::into)
    }
}

impl<I, M> GetKeyValue<&I> for Dictionary<I, M> 
where
    M: for<'a> GetKeyValue<&'a I>, 
    for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
    for<'a> <M as KeyedRef>::KeyRef<'a>: Into<&'a I>,
{
    fn get_key_value(&self, key: &I) -> Option<(Self::KeyRef<'_>, Self::ItemRef<'_>)> {
        self.words
        .get_key_value(key)
        .map(|(k, v)| (k.into(), v.into()))
    }
}

impl<I, M> CollectionMut for Dictionary<I, M> {
    type ItemMut<'a> = &'a mut Word where Self: 'a;

    cc_traits::covariant_item_mut!();
}

impl<I, M> SimpleCollectionMut for Dictionary<I, M> {
    cc_traits::simple_collection_mut!();
}

impl<I, M> GetMut<&I> for Dictionary<I, M>
where
    M: for<'a> GetMut<&'a I>,
    for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
    for<'a> <M as CollectionMut>::ItemMut<'a>: Into<&'a mut Word>,
{
    fn get_mut(&mut self, key: &I) -> Option<Self::ItemMut<'_>> {
        self.words.get_mut(key).map(Into::into)
    }
}

impl<I, M> MapInsert<I> for Dictionary<I, M>
where
    M: MapInsert<I, Output = Option<Word>>,
    <M as Collection>::Item: From<Word>,
{
    type Output = Option<Word>;

    fn insert(&mut self, key: I, value: Self::Item) -> Self::Output {
        self.words.insert(key, value.into())
    }
}

impl<I, M> Remove<&I> for Dictionary<I, M> 
where
    M: for<'a> Remove<&'a I>,
    <M as Collection>::Item: Into<Word>,
{
    fn remove(&mut self, key: &I) -> Option<Self::Item> {
        self.words.remove(key).map(Into::into)
    }
}

impl<I, M> Clear for Dictionary<I, M>
where
    M: Clear
{
    fn clear(&mut self) {
        self.words.clear();
    }
}
