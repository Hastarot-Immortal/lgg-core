use crate::{ Dictionary, Word };

use cc_traits::{
	Iter,
	IterMut,
	MapIter,
	MapIterMut,
};

use std::marker::PhantomData;

/// An iterator over shared references to the [`Word`] values stored in a [`Dictionary`].
///
/// This struct is created by the [`words`](Dictionary::words) method on [`Dictionary`].
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
		/// Returns an iterator yielding immutable references to all [`Word`] tokens stored in this dictionary.
    pub fn words(&self) -> Words<'_, M> {
        Words {
            inner: self.words.iter(),
        }
    }
}

/// An iterator over mutable references to the [`Word`] values stored in a [`Dictionary`].
///
/// This struct is created by the [`words_mut`](Dictionary::words_mut) method on [`Dictionary`].
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
	/// Returns an iterator yielding mutable references to all [`Word`] tokens stored in this dictionary.
	pub fn words_mut(&mut self) -> WordsMut<'_, M> {
		WordsMut {
			inner: self.words.iter_mut(),
		}
	}
}

/// An iterator over shared key-value pairs `(&I, &Word)` stored in a [`Dictionary`].
///
/// This struct is created by the [`iter`](MapIter::iter) implementation on [`Dictionary`].
pub struct DictIter<'a, I: 'a, M: MapIter + 'a> {
	inner: M::Iter<'a>,
	_id_type: PhantomData<I>,
}

impl<'a, I, M> Iterator for DictIter<'a, I, M>
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
	type Iter<'a> = DictIter<'a, I, M> where Self: 'a, M: 'a;

	/// Creates an iterator visiting all key-value pairings in arbitrary order.
	fn iter(&self) -> Self::Iter<'_> {
		DictIter {
			inner: self.words.iter(),
			_id_type: PhantomData,
		}
	}
}

/// An iterator over mutable key-value pairs `(&I, &mut Word)` stored in a [`Dictionary`].
///
/// This struct is created by the [`iter_mut`](MapIterMut::iter_mut) implementation on [`Dictionary`].
pub struct DictIterMut<'a, I: 'a, M: MapIterMut + 'a> {
	inner: M::IterMut<'a>,
	_id_type: PhantomData<I>,
}

impl<'a, I, M> Iterator for DictIterMut<'a, I, M>
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
	type IterMut<'a> = DictIterMut<'a, I, M> where Self: 'a, M: 'a;
	
	/// Creates an iterator visiting all key-value pairings in arbitrary order, allowing modification of the underlying words.
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		DictIterMut {
			inner: self.words.iter_mut(),
			_id_type: PhantomData,
		}
	}
}
