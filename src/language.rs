use crate::{Dictionary, PartOfSpeech, Rule, Word};

use std::{
    hash::Hash,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language<I, M=HashMap<I, Word>> {
	dictionary: Dictionary<I, M>,
}

impl<I> Language<I>
where
	I: Hash + Eq,
{
	pub fn new() -> Self {
        Self {
            dictionary: Dictionary::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dictionary: Dictionary::with_capacity(capacity)
        }
    }

    pub fn from_array<const N: usize>(arr: [(I, Word); N]) -> Self {
        <Self as From<[(I, Word); N]>>::from(arr)
    }

    pub fn from_vec(vec: Vec<(I, Word)>) -> Self {
        Self {
            dictionary: Dictionary::from_iter(vec.into_iter()),
        }
    }
}

impl<I, M> From<Dictionary<I, M>> for Language<I, M>
{
    fn from(dictionary: Dictionary<I, M>) -> Self {
        Self { dictionary }
    }
}

impl<I, M, const N: usize> From<[(I, Word); N]> for Language<I, M>
where
    M: From<[(I, Word); N]>,
{
    fn from(arr: [(I, Word); N]) -> Self {
        Self {
            dictionary: Dictionary::from(arr),
        }
    }
}

impl<I, M> FromIterator<(I, Word)> for Language<I, M>
where
    M: FromIterator<(I, Word)>,
{
    fn from_iter<T: IntoIterator<Item = (I, Word)>>(iter: T) -> Language<I, M> {
        Self {
            dictionary: Dictionary::from_iter(iter),
        }
    }
}

impl<I> Deref for Language<I>
where
    I: Hash + Eq,
{
    type Target = Dictionary<I>;

    fn deref(&self) -> &Self::Target {
        &self.dictionary
    }
}

impl<I> DerefMut for Language<I>
where
    I: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dictionary
    }
}

pub trait LanguageBuilder<I: Hash + Eq> {
    fn new() -> Self;
    fn rules(self, rules: Vec<Box<dyn Rule>>) -> Self;
    fn build(&mut self, words: Vec<(I, PartOfSpeech)>) -> Language<I>;
    fn add_words(&mut self, language: &mut Language<I>, words: Vec<(I, PartOfSpeech)>);
    fn transform(&self, language: &mut Language<I>);
}

pub trait RandomLanguageBuilder<I: Hash + Eq>: LanguageBuilder<I> {
	type Seed;
	fn seed(self, seed: Self::Seed) -> Self;
}

#[cfg(test)]
mod language_test {
    use super::*;
    use crate::{ 
        Sound, 
        VoiceLevel,
    };

    fn create_simple_alphabet() -> (Sound, Sound, Sound, Sound) {
        (
            Sound::new('t', VoiceLevel::Voiceless),
            Sound::monophthong('e'),
            Sound::new('w', VoiceLevel::Sonorant),
            Sound::new('z', VoiceLevel::Voice)
        )
    }

    #[test]
    fn insert() {
        let (t, e, w, z) = create_simple_alphabet();

        let mut language = Language::with_capacity(3);
        language.insert(1, Word::from_slice(&[t, e, w], PartOfSpeech::NOUN));
        language.insert(2, Word::from_slice(&[z, e, w], PartOfSpeech::VERB));
        language.insert(3, Word::from_slice(&[t, e, z], PartOfSpeech::PRON));

        assert_eq!(language.get(&1).map(|word| word.to_string()), Some("tew".to_string()));
        assert_eq!(language.get(&2).map(|word| word.to_string()), Some("zew".to_string()));
        assert_eq!(language.get(&3).map(|word| word.to_string()), Some("tez".to_string()));
    }
}