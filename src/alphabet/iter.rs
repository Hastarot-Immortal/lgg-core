use std::{
    ops::{Deref, DerefMut},
    iter::Map,
    vec::IntoIter as VecIntoIter,
    slice::Iter as SliceIter,
};
use crate::{
    Sound, 
    alphabet::{
        Alphabet, 
        alphabet::AlphabetUnit
    },
};

pub struct IntoIter {
    inner: Map<VecIntoIter<AlphabetUnit>, fn(AlphabetUnit) -> Sound>,
}

impl Iterator for IntoIter {
    type Item = Sound;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for Alphabet {
    type Item = Sound;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.storage.into_iter().map(|s| s.sound)
        }
    }
}

pub struct Iter<'a> {
    inner: Map<SliceIter<'a, AlphabetUnit>, fn(&'a AlphabetUnit) -> &'a Sound>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Sound;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a Alphabet {
    type Item = &'a Sound;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.storage.iter().map(|s| &s.sound)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Indexes {
    pub(crate) inner: Vec<usize>,
}

impl Deref for Indexes {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Indexes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl IntoIterator for Indexes {
    type Item = usize;
    type IntoIter = VecIntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Indexes {
    type Item = &'a usize;
    type IntoIter = SliceIter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Indexes {
    type Item = &'a mut usize;
    type IntoIter = std::slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}