use crate::{Dictionary, PartOfSpeech, Rule, Word, collections::FastMap as DefaultMap};

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language<I, M=DefaultMap<I, Word>> {
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
            dictionary: Dictionary::from_iter(vec),
        }
    }
}

impl<I, M> From<Dictionary<I, M>> for Language<I, M> {
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

impl<I, M> Deref for Language<I, M> {
    type Target = Dictionary<I, M>;

    fn deref(&self) -> &Self::Target {
        &self.dictionary
    }
}

impl<I, M> DerefMut for Language<I, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dictionary
    }
}

pub trait LanguageBuilder<T, M=DefaultMap<T, Word>> {
    fn new() -> Self;
    fn build<I: IntoIterator<Item=(T, PartOfSpeech)>>(&mut self, words: I) -> Language<T, M>;
}

pub trait RuleBasedLanguageBuilder {
    fn rules<R, I>(self, rules: I) -> Self
    where
        R: Into<Box<dyn Rule>>,
        I: IntoIterator<Item=R>;
}

pub trait RandomLanguageBuilder {
	type Seed;
	fn seed<S: Into<Self::Seed>>(self, seed: S) -> Self;
}

#[cfg(feature="alphabet")]
pub trait LanguageWithAlphabetBuilder {
    fn alphabet<A: Into<crate::alphabet::Alphabet>>(self, alphabet: A) -> Self;
}

pub trait LanguageExtender<T, M=DefaultMap<T, Word>> {
    fn extend<I: IntoIterator<Item=(T, Word)>>(&mut self, language: &mut Language<T, M>, words: I);
}

pub trait LanguageTransformer<I, M=DefaultMap<I, Word>> {
    fn transform(&mut self, language: &mut Language<I, M>);
    fn transform_create<T, N>(&mut self, language: &Language<I, M>) -> Language<T, N>;
}

#[cfg(test)]
mod language_test {
    use super::*;
    use crate::Sound;

    fn create_simple_alphabet() -> (Sound, Sound, Sound, Sound) {
        (
            Sound::voiceless('t'),
            Sound::vowel('e'),
            Sound::sonorant('w'),
            Sound::voice('z')
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