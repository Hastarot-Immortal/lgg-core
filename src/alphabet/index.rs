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
	impl<K> Sealed for K where K: TryAsSound {}
	impl Sealed for Indexes {}
}

pub trait AlphabetIndex: sealed::Sealed {
	type Output;
	fn get(self, alphabet: &Alphabet) -> Option<&Self::Output>;
	fn index(self, alphabet: &Alphabet) -> &Self::Output;
}

impl AlphabetIndex for usize {
	type Output = Sound;

	fn get(self, alphabet: &Alphabet) -> Option<&Sound> {
		alphabet.storage.get(self).map(|u| &u.sound)
	}

	fn index(self, alphabet: &Alphabet) -> &Sound {
		&alphabet.storage[self].sound
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

impl AlphabetIndexOwned for usize {
	type Owned = Sound;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Sound> {
		alphabet.storage.get(self).map(|u| u.sound)
	}
}

impl<K> AlphabetIndexOwned for K
where 
	K: TryAsSound 
{
	type Owned = Sound;

	fn get_owned(self, alphabet: &Alphabet) -> Option<Sound> {
		search(alphabet, self).copied()
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