use crate::{
    alphabet::{
        iter::{Indexes, VoiceLevelSet},
        index::{AlphabetIndex, AlphabetIndexOwned},
    },
    Sound,
    sound::{AsSound, TryAsSound},
};
use std::ops::Index;

pub type AlphabetKey = [u8; 6];

pub struct Alphabet {
    pub(super) storage: Vec<Sound>,
}

impl From<Vec<Sound>> for Alphabet {
    fn from(mut storage: Vec<Sound>) -> Self {
        storage.sort_by_key(|s| s.as_sound());
        storage.dedup_by_key(|s| s.as_sound());
        storage.shrink_to_fit();
        Self { storage }
    }
}

impl FromIterator<Sound> for Alphabet {
    fn from_iter<T: IntoIterator<Item = Sound>>(sounds: T) -> Self {
        let storage: Vec<_> = sounds.into_iter().collect();
        Self::from(storage)
    }
}

impl<const N: usize> From<[Sound; N]> for Alphabet {
    fn from(sounds: [Sound; N]) -> Self {
        Self::from_iter(sounds)
    }
}

impl Alphabet {
    pub fn get<I: AlphabetIndex>(&self, idx: I) -> Option<&I::Output> {
        idx.get(self)
    }

    pub fn get_owned<I: AlphabetIndexOwned>(&self, idx: I) -> Option<I::Owned> {
        idx.get_owned(self)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn indexes(&self) -> Indexes {
        Indexes {
            inner: (0..self.len()).collect(),
        }
    }

    pub fn indexes_by<F: Fn(&Sound) -> bool>(&self, f: F) -> Indexes {
        Indexes {
            inner: self
                .storage
                .iter()
                .enumerate()
                .filter_map(|(i, s)| if f(&s) { Some(i) } else { None })
                .collect(),
        }
    }

    pub fn indexes_by_voice_level<I: Into<VoiceLevelSet>>(&self, levels: I) -> Indexes {
        let vl_set = levels.into();
        Indexes {
            inner: self.storage
                .iter()
                .enumerate()
                .filter_map(|(i, s)| {
                    let shift = s.voice_level() as u8;
                    if ((1 << shift) & vl_set.0) != 0 {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect()
        }
    }
}

impl<I> Index<I> for Alphabet 
where
    I: AlphabetIndex
{
    type Output = I::Output;
    fn index(&self, idx: I) -> &Self::Output {
        idx.index(self)
    }
}

pub(super) fn search<K: TryAsSound>(alphabet: &Alphabet, key: K) -> Option<&Sound> {
    let key = key.try_as_sound().ok()?;
    let idx = alphabet.storage.binary_search_by_key(&key, |s| s.as_sound()).ok()?;
    Some(&alphabet.storage[idx])
}
