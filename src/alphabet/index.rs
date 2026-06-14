use crate::{
	Sound, 
	sound::TryAsSound,
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
	impl<K> Sealed for K where K: TryAsSound {}
	impl Sealed for Indexes {}
	impl Sealed for Vec<usize> {}
	impl<'a> Sealed for &'a [usize] {}
	impl<K> Sealed for Vec<K> where K: TryAsSound {}
}

pub trait AlphabetIndex: sealed::Sealed {
	type Output;
	fn get(self, alphabet: &Alphabet) -> Option<&Self::Output>;
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
	K: TryAsSound, 
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

pub trait AlphabetIndexOwned: sealed::Sealed {
	type Owned;
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
	K: TryAsSound, 
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
