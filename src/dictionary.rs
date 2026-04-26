use crate::Word;
#[cfg(feature="affix")]
use crate::Affix;

use std::{
    collections::HashMap, 
    hash::Hash,
    ops::{Deref, DerefMut},
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary<Key: Hash + Eq> {
    words: HashMap<Key, Word>,
    #[cfg(feature="affix")]
    pub affixes: HashMap<Key, Affix>,
}

impl<I> Dictionary<I>
where
    I: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            #[cfg(feature="affix")]
            affixes: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            words: HashMap::with_capacity(capacity),
            #[cfg(feature="affix")]
            affixes: HashMap::new(),
        }
    }
}

impl<Key> Deref for Dictionary<Key>
where
    Key: Hash + Eq,
{
    type Target = HashMap<Key, Word>;

    fn deref(&self) -> &Self::Target {
        &self.words
    }
}

impl<Key> DerefMut for Dictionary<Key>
where
    Key: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.words
    }
}