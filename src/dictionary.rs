use crate::Word;

use std::{
    collections::HashMap, 
    hash::Hash,
    ops::{Deref, DerefMut},
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary<I: Hash + Eq> {
    words: HashMap<I, Word>,
}

impl<I> Dictionary<I>
where
    I: Hash + Eq,
{
    pub fn new() -> Self {
        Self { words: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { words: HashMap::with_capacity(capacity) }
    }
}

impl<I> Deref for Dictionary<I>
where
    I: Hash + Eq,
{
    type Target = HashMap<I, Word>;

    fn deref(&self) -> &Self::Target {
        &self.words
    }
}

impl<I> DerefMut for Dictionary<I>
where
    I: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.words
    }
}