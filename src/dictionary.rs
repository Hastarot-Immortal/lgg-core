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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary<I, M = FastMap<I, Word>> {
    pub(crate) words: M,
    _id_type: PhantomData<I>,
}

impl<I> Dictionary<I>
where
    I: Hash + Eq,
{
    pub fn new() -> Self {
        Self { 
            words: FastMap::new(), 
            _id_type: PhantomData
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            words: FastMap::with_capacity(capacity), 
            _id_type: PhantomData
        }
    }

    pub fn from_array<const N: usize>(arr: [(I, Word); N]) -> Self {
        <Self as From<[(I, Word); N]>>::from(arr)
    }

    pub fn from_vec(vec: Vec<(I, Word)>) -> Self {
        Self {
            words: FastMap::from_iter(vec.into_iter()),
            _id_type: PhantomData
        }
    }
}

impl<I, M> Dictionary<I, M> {
    pub fn len(&self) -> usize 
    where
        M: Len
    {
        Len::len(self)
    }

    pub fn get(&self, key: &I) -> Option<&Word> 
    where
        M: for<'a> Get<&'a I>,
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
    {
        Get::get(self, key)
    }

    pub fn contains(&self, key: &I) -> bool
    where
        M: for<'a> Get<&'a I>,
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
    {
        Get::contains(self, key)
    }

    pub fn get_key_value(&self, key: &I) -> Option<(&I, &Word)>
    where
        M: for<'a> GetKeyValue<&'a I>, 
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
        for<'a> <M as KeyedRef>::KeyRef<'a>: Into<&'a I>,
    {
        GetKeyValue::get_key_value(self, key)
    }

    pub fn get_mut(&mut self, key: &I) -> Option<&mut Word>
    where
        M: for<'a> GetMut<&'a I>,
        for<'a> <M as CollectionRef>::ItemRef<'a>: Into<&'a Word>,
        for<'a> <M as CollectionMut>::ItemMut<'a>: Into<&'a mut Word>,
    {
        GetMut::get_mut(self, key)
    }

    pub fn insert(&mut self, key: I, value: Word) -> Option<Word> 
    where
        M: MapInsert<I, Output = Option<Word>>,
        <M as Collection>::Item: From<Word>,
    {
        MapInsert::insert(self, key, value)
    }

    pub fn remove(&mut self, key: &I) -> Option<Word> 
    where
        M: for<'a> Remove<&'a I>,
        <M as Collection>::Item: Into<Word>,
    {
        Remove::remove(self, key)
    }

    pub fn clear(&mut self)
    where
        M: Clear
    {
        Clear::clear(self)
    }

    pub fn iter(&self) -> crate::DictionaryMapIter<'_, I, M>
    where
        M: MapIter,
        for<'a> M::KeyRef<'a>: Into<&'a I>,
        for<'a> M::ItemRef<'a>: Into<&'a Word>,
    {
        MapIter::iter(self)
    }

    pub fn iter_mut(&mut self) -> crate::DictionaryMapIterMut<'_, I, M>
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

#[cfg(test)]
mod dictionary_test {
    use super::*;
    use crate::{ 
        Sound, 
        VoiceLevel, 
        PartOfSpeech
    };
    use cc_traits::{Map, MapMut};

    fn create_simple_alphabet() -> (Sound, Sound, Sound, Sound) {
        (
            Sound::new('s', VoiceLevel::Voiceless),
            Sound::monophthong('a'),
            Sound::new('m', VoiceLevel::Sonorant),
            Sound::new('v', VoiceLevel::Voice)
        )
    }

    fn check_if_is_empty<I, M: Map<I, Word>>(map: &M) -> bool {
        map.is_empty()
    }

    fn fill_map<M: MapMut<u32, Word>>(map: &mut M) {
        let (s, a, m, v) = create_simple_alphabet();

        map.insert(1, Word::from_slice(&[m, a, s, s], PartOfSpeech::NOUN));
        map.insert(2, Word::from_slice(&[s, a, m], PartOfSpeech::NOUN));
        map.insert(3, Word::from_slice(&[s, a, m, s, a], PartOfSpeech::NOUN));
        map.insert(4, Word::from_slice(&[v, a, s, a], PartOfSpeech::NOUN));
    }

    #[test] 
    fn map() {
        let dict: Dictionary<String> = Dictionary::new();
        let is_empty = check_if_is_empty(&dict);
        assert_eq!(is_empty, true);
    }

    #[test]
    fn map_mut() {
        let (s, a, m, _) = create_simple_alphabet();

        let mut dict: Dictionary<u32> = Dictionary::new();
        fill_map(&mut dict);
        assert_eq!(dict.get(&2), Some(&Word::from_slice(&[s, a, m], PartOfSpeech::NOUN)));
    }

    #[test]
    fn from() {
        let (s, a, m, v) = create_simple_alphabet();

        let arr = [
            (-1i8, Word::from_slice(&[m, a, v], PartOfSpeech::NUM)),
            (0i8, Word::from_slice(&[s, a, v], PartOfSpeech::VERB)),
            (1i8, Word::from_slice(&[s, a, m], PartOfSpeech::CONJ)),
        ];

        let dict = Dictionary::from_array(arr);
        assert_eq!(dict.get(&0).map(|word: &Word| word.to_string()), Some("sav".to_string()))
    }

    #[test]
    fn from_iter() {
        let (s, a, m, v) = create_simple_alphabet();

        let vec = vec![
            (-1i8, Word::from_slice(&[m, a, v], PartOfSpeech::NUM)),
            (0i8, Word::from_slice(&[s, a, v], PartOfSpeech::VERB)),
            (1i8, Word::from_slice(&[s, a, m], PartOfSpeech::CONJ)),
        ];

        let dict: Dictionary<i8, FastMap<i8, Word>> = Dictionary::from_iter(vec);
        assert_eq!(dict.get(&-1).map(|word: &Word| word.to_string()), Some("mav".to_string()))
    }

    #[test]
    fn remove() {
        let mut dict = Dictionary::new();
        fill_map(&mut dict);
        let word = dict.remove(&1);
        assert_eq!(word.map(|w| w.to_string()), Some("mass".to_string()));
        assert_eq!(dict.get(&1), None);
    }

    #[test]
    fn clear() {
        let mut dict = Dictionary::new();
        fill_map(&mut dict);
        assert_eq!(dict.is_empty(), false);
        dict.clear();
        assert_eq!(dict.is_empty(), true);
    }

    #[test]
    fn words() {
        let mut dict = Dictionary::new();
        fill_map(&mut dict);
        let mut words = dict.words();
        assert!(words.next().is_some());
    }

    #[test]
    fn words_mut() {
        let (s, a, m, v) = create_simple_alphabet();
        let mut dict = Dictionary::new();
        dict.insert(1, Word::from_slice(&[m, a, s, s], PartOfSpeech::NOUN));
        for w in dict.words_mut() {
            w.push(v);
        }
        let mut iter = dict.iter();
        assert_eq!(iter.next(), Some((&1, &Word::from_slice(&[m, a, s, s, v], PartOfSpeech::NOUN))));
    }
}