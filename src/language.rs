use crate::{Dictionary, PartOfSpeech, Rule};

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language<I: Hash + Eq> {
	dictionary: Dictionary<I>,
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