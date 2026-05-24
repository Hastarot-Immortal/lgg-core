pub mod alphabet;
pub mod iter;
pub mod index;

pub use alphabet::Alphabet;
pub use iter::{IntoIter, Iter, Indexes};
pub use index::{AlphabetIndex, AlphabetIndexOwned};

#[cfg(test)]
mod alphabet_test {
    use super::*;
    use crate::{Sound, VoiceLevel};
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
        let indexes = ALPHABET.indexes_by(|s| s.voice_level() == VoiceLevel::Sonorant);
        assert_eq!(indexes.len(), 5);
        assert!(ALPHABET.get_owned(indexes).is_some());
    }
}
