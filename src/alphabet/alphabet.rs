use crate::{
    alphabet::{
        iter::{Indexes, VoiceLevelSet},
        index::{AlphabetIndex, AlphabetIndexOwned},
    },
    Sound,
    sound::{AsBytesForSound, TryAsBytesForSound},
};
use std::ops::Index;

/// An ordered pool of unique [`Sound`] instances acting as an alphabet framework.
///
/// The alphabet automatically sorts and deduplicates sounds upon creation, providing 
/// a reliable system for indexing, bulk lookups, and phonetic classification filtering.
///
/// ```
/// use lgg_core::{alphabet::Alphabet, Sound};
///
/// let alphabet = Alphabet::from([
///     Sound::vowel('a'),
///     Sound::voiceless('s'),
///     Sound::vowel('a'), // Will be deduplicated
/// ]);
/// 
/// assert_eq!(alphabet.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alphabet {
    pub(super) storage: Vec<Sound>,
}

impl From<Vec<Sound>> for Alphabet {
    fn from(mut storage: Vec<Sound>) -> Self {
        storage.sort_by_key(|s| s.as_bytes_for_sound());
        storage.dedup_by_key(|s| s.as_bytes_for_sound());
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
    /// Safely fetches a reference out of the alphabet based on a custom identifier index.
    ///
    /// Returns [`None`] if the target index pattern is out of bounds or the sound is missing.
    ///
    /// ```
    /// use lgg_core::{alphabet::Alphabet, Sound};
    ///
    /// let alphabet = Alphabet::from([Sound::vowel('a')]);
    /// 
    /// assert!(alphabet.get(0).is_some());
    /// assert!(alphabet.get('x').is_none());
    /// ```
    pub fn get<I: AlphabetIndex>(&self, idx: I) -> Option<&I::Output> {
        idx.get(self)
    }

    /// Safely fetches a copied or synthesized owned representation from the alphabet.
    ///
    /// Returns [`None`] if any part of the target index sequence falls out of bounds.
    ///
    /// ```
    /// use lgg_core::{alphabet::Alphabet, Sound};
    ///
    /// let alphabet = Alphabet::from([Sound::vowel('a'), Sound::sonorant('m')]);
    /// let indexes = alphabet.indexes();
    /// 
    /// let sounds_vec = alphabet.get_owned(indexes).unwrap();
    /// assert_eq!(sounds_vec.len(), 2);
    /// ```
    pub fn get_owned<I: AlphabetIndexOwned>(&self, idx: I) -> Option<I::Owned> {
        idx.get_owned(self)
    }

    /// Returns the total number of unique phonetic sounds present in the alphabet.
    ///
    /// ```
    /// use lgg_core::{alphabet::Alphabet, Sound};
    ///
    /// let alphabet = Alphabet::from([Sound::vowel('e')]);
    /// assert_eq!(alphabet.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the alphabet contains no phonetic sounds.
    ///
    /// ```
    /// use lgg_core::alphabet::Alphabet;
    ///
    /// let alphabet = Alphabet::from([]);
    /// assert!(alphabet.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Generates a comprehensive collection of numerical position keys tracking every element inside this alphabet.
    ///
    /// ```
    /// use lgg_core::{alphabet::Alphabet, Sound};
    ///
    /// let alphabet = Alphabet::from([Sound::vowel('a'), Sound::vowel('i')]);
    /// let indexes = alphabet.indexes();
    /// 
    /// assert_eq!(indexes.len(), 2);
    /// ```
    pub fn indexes(&self) -> Indexes {
        Indexes {
            inner: (0..self.len()).collect(),
        }
    }
                            
    /// Scans the alphabet pool using a closure predicate, collecting matching position indices.
    ///
    /// ```
    /// use lgg_core::{alphabet::Alphabet, Sound, VoiceLevel};
    ///
    /// let alphabet = Alphabet::from([Sound::vowel('a'), Sound::voiceless('t')]);
    /// let vowels = alphabet.indexes_by(|s| s.voice_level() == VoiceLevel::Vowel);
    /// 
    /// assert_eq!(vowels.len(), 1);
    /// ```
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

    /// Scans the alphabet using an efficient bitmask lookup, collecting positions of items matching specific voice categories.
    ///
    /// Useful for isolating sound profiles such as matching all vowels or sonorants in one sweep.
    ///
    /// ```
    /// use lgg_core::{alphabet::Alphabet, Sound, VoiceLevel};
    ///
    /// let alphabet = Alphabet::from([Sound::vowel('a'), Sound::sonorant('m'), Sound::voiceless('s')]);
    /// let targets = alphabet.indexes_by_voice_level([VoiceLevel::Vowel, VoiceLevel::Sonorant]);
    /// 
    /// assert_eq!(targets.len(), 2);
    /// ```
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

    /// Direct shortcut accessor indexing into the alphabet. 
    ///
    /// # Panics
    ///
    /// Panics if the location query misses or is outside the valid range.
    /// Prefer calling [`Alphabet::get`] for graceful fallback handling.
    fn index(&self, idx: I) -> &Self::Output {
        idx.index(self)
    }
}

pub(super) fn search<K: TryAsBytesForSound>(alphabet: &Alphabet, key: K) -> Option<&Sound> {
    let key = key.try_as_bytes_for_sound().ok()?;
    let idx = alphabet.storage.binary_search_by_key(&key, |s| s.as_bytes_for_sound()).ok()?;
    Some(&alphabet.storage[idx])
}
