use crate::{
	Sound, 
	sound::TryAsBytesForSound,
	alphabet::{
		Alphabet, 
		iter::Indexes,
		alphabet::search,
	}
};

mod sealed {
	use super::*;
	pub trait Sealed {}

	impl Sealed for usize {}
	impl<'a> Sealed for &'a usize {}
	impl Sealed for Option<usize> {}
	impl Sealed for Option<&usize> {}
	impl<K> Sealed for K where K: TryAsBytesForSound {}
	impl Sealed for Indexes {}
	impl Sealed for Vec<usize> {}
	impl<'a> Sealed for &'a [usize] {}
	impl<K> Sealed for Vec<K> where K: TryAsBytesForSound {}
}

/// A sealed trait powering polymorphic immutable lookups inside an [`Alphabet`].
///
/// It allows querying the alphabet using raw numeric positions, optional values, 
/// characters (`char`), or string tokens (`&str`).
pub trait AlphabetIndex: sealed::Sealed {
	/// The mapped resulting reference type produced by an indexing operation.
	type Output;

	/// Safely attempts to resolve a shared reference matching this query inside the alphabet.
  ///
  /// Returns [`None`] if the query position falls outside valid allocation bounds or if a string search misses.
	fn get(self, alphabet: &Alphabet) -> Option<&Self::Output>;

	/// Force-indexes into the alphabet, returning a shared reference.
  ///
  /// # Panics
  ///
  /// Panics if the location query misses or is outside the valid range.
	fn index(self, alphabet: &Alphabet) -> &Self::Output;
}

impl AlphabetIndex for usize {
	type Output = Sound;

	fn get(self, alphabet: &Alphabet) -> Option<&Sound> {
		alphabet.storage.get(self)
	}

	fn index(self, alphabet: &Alphabet) -> &Sound {
		&alphabet.storage[self]
	}
}

impl<'a> AlphabetIndex for &'a usize {
	type Output = Sound;

	fn get(self, alphabet: &Alphabet) -> Option<&Sound> {
		(*self).get(alphabet)
	}

	fn index(self, alphabet: &Alphabet) -> &Sound {
		(*self).index(alphabet)
	}
}

impl AlphabetIndex for Option<usize> {
	type Output = Sound;

	fn get(self, alphabet: &Alphabet) -> Option<&Sound> {
		self.map(|idx| alphabet.storage.get(idx)).flatten()
	}

	fn index(self, alphabet: &Alphabet) -> &Sound {
		let idx = self.expect("index must not be None");
		idx.index(alphabet)
	}
}

impl AlphabetIndex for Option<&usize> {
	type Output = Sound;

	fn get(self, alphabet: &Alphabet) -> Option<&Sound> {
		self.copied().get(alphabet)
	}

	fn index(self, alphabet: &Alphabet) -> &Sound {
		self.copied().index(alphabet)
	}
}

impl<K> AlphabetIndex for K
where 
	K: TryAsBytesForSound, 
{
	type Output = Sound;

	fn get(self, alphabet: &Alphabet) -> Option<&Sound> {
		search(alphabet, self)
	}

	fn index(self, alphabet: &Alphabet) -> &Sound {
		search(alphabet, self)
			.expect("there aren't such sound in alphabet")
	}
}

/// A sealed trait powering slice extraction, bulk cloning, and complex sound collection queries.
///
/// It supports retrieving multiple sounds at once via vectors, slices, or [`Indexes`] trackers.
pub trait AlphabetIndexOwned: sealed::Sealed {
	/// The owned collection type synthesized from evaluating this index pattern against the alphabet.
	type Owned;

	/// Evaluates the sequence criteria against the alphabet pool, reconstructing an owned copy of matched items.
	///
	/// Returns [`None`] if any fragment inside the collective query sequence fails evaluation metrics.
	fn get_owned(self, alphabet: &Alphabet) -> Option<Self::Owned>;
}

impl<I> AlphabetIndexOwned for I
where
	I: AlphabetIndex,
	I::Output: Clone,
{
	type Owned = I::Output;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Self::Owned> {
		self.get(alphabet).cloned()
	}
}

impl AlphabetIndexOwned for Indexes {
	type Owned = Vec<Sound>;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Vec<Sound>> {
		let mut res = Vec::with_capacity(self.len());
		for i in self {
			if let Some(s) = alphabet.get(i) {
				res.push(*s);
			} else {
				return None;
			}
		}
		Some(res)
	}
}

impl<'a> AlphabetIndexOwned for &'a [usize] {
	type Owned = Vec<Sound>;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Vec<Sound>> {
		Indexes { inner: self.to_owned() }.get_owned(alphabet)
	}
}

impl AlphabetIndexOwned for Vec<usize> {
	type Owned = Vec<Sound>;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Vec<Sound>> {
		Indexes { inner: self }.get_owned(alphabet)
	}
}

impl<K> AlphabetIndexOwned for Vec<K> 
where 
	K: TryAsBytesForSound, 
{
	type Owned = Vec<Sound>;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Vec<Sound>> {
		let mut res = Vec::with_capacity(self.len());
		for sound in self {
			if let Some(s) = search(alphabet, sound) {
				res.push(*s);
			} else {
				return None;
			}
		}
		Some(res)
	}
}
