pub mod alphabet;
pub mod iter;
pub mod key;
pub mod index;

pub use alphabet::Alphabet;
pub use iter::{IntoIter, Iter, Indexes};
pub use key::{AlphabetKey, AsKey};
pub use index::{AlphabetIndex, AlphabetIndexOwned};

#[cfg(test)]
mod alphabet_test {
    use super::*;
    use crate::{Sound, VoiceLevel};
    use std::sync::LazyLock;

    static ALPHABET: LazyLock<Alphabet> = LazyLock::new(|| {
        Alphabet::from([
            Sound::monophthong('a'),
            Sound::monophthong('e'),
            Sound::monophthong('i'),
            Sound::monophthong('u'),
            Sound::monophthong('o'),
            Sound::monophthong('ʊ'),
            Sound::diphthong(['o', 'u']),
            Sound::diphthong(['e', 'j']),
            Sound::new('w', VoiceLevel::Sonorant),
            Sound::new('m', VoiceLevel::Sonorant),
            Sound::new('r', VoiceLevel::Sonorant),
            Sound::new('j', VoiceLevel::Sonorant),
            Sound::new('n', VoiceLevel::Sonorant),
            Sound::new('v', VoiceLevel::Voice),
            Sound::new('z', VoiceLevel::Voice),
            Sound::new('d', VoiceLevel::Voice),
            Sound::new('b', VoiceLevel::Voice),
            Sound::new('ð', VoiceLevel::Voice),
            Sound::new('ʒ', VoiceLevel::Voice),
            Sound::new('β', VoiceLevel::Voice),
            Sound::new('s', VoiceLevel::Voiceless),
            Sound::new('t', VoiceLevel::Voiceless),
            Sound::new('f', VoiceLevel::Voiceless),
            Sound::new('k', VoiceLevel::Voiceless),
            Sound::new('x', VoiceLevel::Voiceless),
            Sound::new('ʃ', VoiceLevel::Voiceless),
            Sound::new('θ', VoiceLevel::Voiceless),
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
