use crate::{ Dictionary, Word };

use cc_traits::{
	Iter,
	IterMut,
	MapIter,
	MapIterMut,
};

use std::marker::PhantomData;

pub struct Words<'a, M: Iter + 'a> {
    inner: M::Iter<'a>,
}

impl<'a, M: Iter + 'a> Iterator for Words<'a, M>
where
    M::ItemRef<'a>: Into<&'a Word>,
{
    type Item = &'a Word;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Into::into)
    }
}

impl<I, M> Dictionary<I, M>
where
    M: Iter,
    for<'a> M::ItemRef<'a>: Into<&'a Word>,
{
    pub fn words(&self) -> Words<'_, M> {
        Words {
            inner: self.words.iter(),
        }
    }
}

pub struct WordsMut<'a, M: IterMut + 'a> {
    inner: M::IterMut<'a>,
}

impl<'a, M: IterMut + 'a> Iterator for WordsMut<'a, M>
where
	M::ItemMut<'a>: Into<&'a mut Word>,
{
	type Item = &'a mut Word;

	fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Into::into)
    }
}

impl<I, M> Dictionary<I, M>
where
	M: IterMut,
	for<'a> M::ItemMut<'a>: Into<&'a mut Word>,
{
	pub fn words_mut(&mut self) -> WordsMut<'_, M> {
		WordsMut {
			inner: self.words.iter_mut(),
		}
	}
}

pub struct DictionaryMapIter<'a, I: 'a, M: MapIter + 'a> {
	inner: M::Iter<'a>,
	_id_type: PhantomData<I>,
}

impl<'a, I, M> Iterator for DictionaryMapIter<'a, I, M>
where
	I: 'a,
	M: MapIter + 'a,
	M::KeyRef<'a>: Into<&'a I>,
    M::ItemRef<'a>: Into<&'a Word>,
{
    type Item = (&'a I, &'a Word);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.into(), v.into()))
    }
}

impl<I, M> MapIter for Dictionary<I, M>
where
	M: MapIter,
	for<'a> M::KeyRef<'a>: Into<&'a I>,
    for<'a> M::ItemRef<'a>: Into<&'a Word>,
{
	type Iter<'a> = DictionaryMapIter<'a, I, M> where Self: 'a, M: 'a;

	fn iter(&self) -> Self::Iter<'_> {
		DictionaryMapIter {
			inner: self.words.iter(),
			_id_type: PhantomData,
		}
	}
}

pub struct DictionaryMapIterMut<'a, I: 'a, M: MapIterMut + 'a> {
	inner: M::IterMut<'a>,
	_id_type: PhantomData<I>,
}

impl<'a, I, M> Iterator for DictionaryMapIterMut<'a, I, M>
where
	I: 'a,
	M: MapIterMut + 'a,
	M::KeyRef<'a>: Into<&'a I>,
    M::ItemMut<'a>: Into<&'a mut Word>,
{
    type Item = (&'a I, &'a mut Word);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.into(), v.into()))
    }
}

impl<I, M> MapIterMut for Dictionary<I, M>
where
	M: MapIterMut,
	for<'a> M::KeyRef<'a>: Into<&'a I>,
    for<'a> M::ItemMut<'a>: Into<&'a mut Word>,
{
	type IterMut<'a> = DictionaryMapIterMut<'a, I, M> where Self: 'a, M: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		DictionaryMapIterMut {
			inner: self.words.iter_mut(),
			_id_type: PhantomData,
		}
	}
}