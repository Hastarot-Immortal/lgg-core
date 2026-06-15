//! Phonetic alphabet framework and sequence coordinate lookups.
//!
//! This module provides the central [`Alphabet`] pool struct along with components 
//! for polymorphic safe array lookups (`index`), item iteration, and acoustic 
//! classification masking (`iter`).
//!
//! # Examples
//!
//! ```
//! use lgg_core::alphabet::{Alphabet, Indexes, VoiceLevelSet};
//! use lgg_core::{Sound, VoiceLevel};
//!
//! // Instantiating a framework pool of phonetic sounds
//! let alphabet = Alphabet::from([
//!     Sound::vowel('a'),
//!     Sound::sonorant('m'),
//!     Sound::voiceless('p'),
//! ]);
//!
//! // Isolate specific coordinates using an acoustic classification mask
//! let consonants: Indexes = alphabet.indexes_by_voice_level(VoiceLevelSet::CONSONANTS);
//! assert_eq!(consonants.len(), 2);
//! ```

pub mod alphabet;
pub mod iter;
pub mod index;

pub use alphabet::Alphabet;
pub use iter::{IntoIter, Iter, Indexes, VoiceLevelSet};
pub use index::{AlphabetIndex, AlphabetIndexOwned};

#[cfg(test)]
mod alphabet_test {
    use super::*;
    use crate::{Sound, VoiceLevel::*};
    use std::sync::LazyLock;

    static ALPHABET: LazyLock<Alphabet> = LazyLock::new(|| {
        Alphabet::from([
            Sound::vowel('a'),
            Sound::vowel('e'),
            Sound::vowel('i'),
            Sound::vowel('u'),
            Sound::vowel('o'),
            Sound::vowel('ʊ'),
            Sound::try_vowel(['o', 'u']).unwrap(),
            Sound::try_vowel(['e', 'j']).unwrap(),
            Sound::sonorant('w'),
            Sound::sonorant('m'),
            Sound::sonorant('r'),
            Sound::sonorant('j'),
            Sound::sonorant('n'),
            Sound::voice('v'),
            Sound::voice('z'),
            Sound::voice('d'),
            Sound::voice('b'),
            Sound::voice('ð'),
            Sound::voice('ʒ'),
            Sound::voice('β'),
            Sound::voiceless('s'),
            Sound::voiceless('t'),
            Sound::voiceless('f'),
            Sound::voiceless('k'),
            Sound::voiceless('x'),
            Sound::voiceless('ʃ'),
            Sound::voiceless('θ'),
        ])
    });

    #[test]
    fn get_is_some() {
        assert!(ALPHABET.get('f').is_some());
    }

    #[test]
    fn get_is_none() {
        assert!(ALPHABET.get("au").is_none());
    }

    #[test]
    fn indexes() {
        let indexes = ALPHABET.indexes();
        assert_eq!(indexes.len(), ALPHABET.len());
        assert_eq!(
            indexes,
            Indexes {
                inner: (0..ALPHABET.len()).collect::<Vec<usize>>()
            }
        );
    }

    #[test]
    fn indexes_by() {
        let indexes = ALPHABET.indexes_by(|s| s.voice_level() == Sonorant);
        assert_eq!(indexes.len(), 5);
        assert!(ALPHABET.get_owned(indexes).is_some());
    }

    static SIMPLE_ALPHABET: LazyLock<Alphabet> = LazyLock::new(|| {
        Alphabet::from([
            Sound::vowel('a'),
            Sound::vowel('e'),
            Sound::sonorant('w'),
            Sound::sonorant('m'),
            Sound::voice('v'),
            Sound::voice('z'),
            Sound::voiceless('s'),
            Sound::voiceless('t'),
        ])
    });

    #[test]
    fn indexes_by_voice_level_from_single() {
        let indexes = SIMPLE_ALPHABET.indexes_by_voice_level(Sonorant);
        assert_eq!(indexes.len(), 2);
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some());
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some_and(|v| v.contains(&Sound::sonorant('w'))));
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some_and(|v| v.contains(&Sound::sonorant('m'))));
    }

    fn test_array_with_sonorants_and_vowels(indexes: Indexes) {
        assert_eq!(indexes.len(), 4);
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some());
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some_and(|v| v.contains(&Sound::sonorant('w'))));
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some_and(|v| v.contains(&Sound::sonorant('m'))));
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some_and(|v| v.contains(&Sound::vowel('a'))));
        assert!(SIMPLE_ALPHABET.get_owned(indexes.clone()).is_some_and(|v| v.contains(&Sound::vowel('e'))));
    }

    #[test]
    fn indexes_by_voice_level_from_array() {
        let indexes = SIMPLE_ALPHABET.indexes_by_voice_level([Sonorant, Vowel]);
        test_array_with_sonorants_and_vowels(indexes);
    }

    #[test]
    fn indexes_by_voice_level_from_slice() {
        let indexes = SIMPLE_ALPHABET.indexes_by_voice_level([Sonorant, Vowel].as_slice());
        test_array_with_sonorants_and_vowels(indexes);
    }

    #[test]
    fn voice_level_set() {
        let vl_set = VoiceLevelSet::ALL - Sonorant;
        let vl_set = vl_set & [Sonorant, Vowel, Voice];
        let vl_set = vl_set | Voiceless;
        assert_eq!(!vl_set,  VoiceLevelSet::from([Breathy, Creaky, Sonorant]));
    }
}
