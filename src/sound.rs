use std::fmt::{Display, Formatter, Error as FmtError};

/// Represents a distinct linguistic sound (**phoneme**) within a word.
///
/// Under the hood, it consist of a stack-allocated, fixed-size byte array (`[u8; 6]`) 
/// paired alongside its [`VoiceLevel`].
/// 
/// ```
/// use lgg_core::Sound;
///
/// let e = Sound::vowel('ɛ');
/// let ph = Sound::try_breathy("pʰ");
/// 
/// assert_eq!(e, "ɛ");
/// assert_eq!(ph.map(|s| s.to_string()), Ok("pʰ".to_string()));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sound {
    bytes: [u8; 6],
    level: VoiceLevel,
}

impl Sound {
    /// Creates a new `Sound` from a type that guarantees a infallible conversion.
    pub fn from<I: AsBytesForSound>(symbol: I, level: VoiceLevel) -> Self {
        Self {
            bytes: symbol.as_bytes_for_sound(),
            level,
        }
    }

    /// Attempts to create a `Sound` from a type that might fail conversion.
    ///
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_from<I: TryAsBytesForSound>(symbol: I, level: VoiceLevel) -> Result<Self, TryAsSoundError> {
        Ok(Self {
            bytes: symbol.try_as_bytes_for_sound()?,
            level,
        })
    }

    /// Creates a new sound with a [`VoiceLevel::Vowel`] classification.
    pub fn vowel<I: AsBytesForSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Vowel)
    }

    /// Attempts to create a new sound with a [`VoiceLevel::Vowel`] classification.
    /// 
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_vowel<I: TryAsBytesForSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Vowel)
    }

    /// Creates a new sound with a [`VoiceLevel::Sonorant`] classification.
    pub fn sonorant<I: AsBytesForSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Sonorant)
    }

    /// Attempts to create a new sound with a [`VoiceLevel::Sonorant`] classification.
    /// 
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_sonorant<I: TryAsBytesForSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Sonorant)
    }

    /// Creates a new sound with a [`VoiceLevel::Voice`] classification.
    pub fn voice<I: AsBytesForSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Voice)
    }

    /// Attempts to create a new sound with a [`VoiceLevel::Voice`] classification.
    /// 
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_voice<I: TryAsBytesForSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Voice)
    }

    /// Creates a new sound with a [`VoiceLevel::Creaky`] classification.
    pub fn creaky<I: AsBytesForSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Creaky)
    }

    /// Attempts to create a new sound with a [`VoiceLevel::Creaky`] classification.
    /// 
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_creaky<I: TryAsBytesForSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Creaky)
    }

    /// Creates a new sound with a [`VoiceLevel::Breathy`] classification.
    pub fn breathy<I: AsBytesForSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Breathy)
    }

    /// Attempts to create a new sound with a [`VoiceLevel::Breathy`] classification.
    /// 
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_breathy<I: TryAsBytesForSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Breathy)
    }

    /// Creates a new sound with a [`VoiceLevel::Voiceless`] classification.
    pub fn voiceless<I: AsBytesForSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Voiceless)
    }

    /// Attempts to create a new sound with a [`VoiceLevel::Voiceless`] classification.
    /// 
    /// # Errors
    ///
    /// Returns [`TryAsSoundError`] if the `symbol` is too large.
    pub fn try_voiceless<I: TryAsBytesForSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Voiceless)
    }

    /// Returns the [`VoiceLevel`] of this sound.
    pub fn voice_level(&self) -> VoiceLevel {
        self.level
    }
}

macro_rules! impl_from {
    ($t:ty) => {
        impl From<($t, VoiceLevel)> for Sound {
            fn from(value: ($t, VoiceLevel)) -> Self {
                Self::from(value.0, value.1)
            }
        }
    };
}

macro_rules! impl_try_from {
    ($t:ty) => {
        impl TryFrom<($t, VoiceLevel)> for Sound {
            type Error = TryAsSoundError;

            fn try_from(value: ($t, VoiceLevel)) -> Result<Self, Self::Error> {
                Self::try_from(value.0, value.1)
            }
        }
    };
}

impl_from!(char);
impl_try_from!(&str);
impl_try_from!([char; 1]);
impl_try_from!([char; 2]);
impl_try_from!([char; 3]);
impl_try_from!(String);

macro_rules! impl_partial_eq {
    ($other:ty) => {
        impl PartialEq<$other> for Sound {
            fn eq(&self, other: &$other) -> bool {
                other.try_as_bytes_for_sound().is_ok_and(|bytes| bytes == self.bytes)
            }
        }

        impl PartialEq<&$other> for Sound {
            fn eq(&self, other: &&$other) -> bool {
                PartialEq::eq(self, *other)
            }
        }

        impl PartialEq<Sound> for $other {
            fn eq(&self, other: &Sound) -> bool {
                self.try_as_bytes_for_sound().is_ok_and(|bytes| bytes == other.bytes)
            }
        }

        impl PartialEq<Sound> for &$other {
            fn eq(&self, other: &Sound) -> bool {
                PartialEq::eq(*self, other)
            }
        }
    };
}

impl_partial_eq!(char);
impl_partial_eq!(str);
impl_partial_eq!([char]);
impl_partial_eq!([char; 1]);
impl_partial_eq!([char; 2]);
impl_partial_eq!([char; 3]);
impl_partial_eq!(String);

impl Display for Sound {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            match str::from_utf8(&self.bytes) {
                Ok(s) => s.trim_end_matches('\0'),
                Err(_) => "invalid symbol to display",
            }
        )
    }
}

/// Represents the phonological voicing/acoustic categorization of a particular [`Sound`].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VoiceLevel {
    /// Produced without vocal cord vibration (e.g., 'p', 't', 'k').
    Voiceless,
    /// Produced with the vocal cords vibrating, but loosely enough to let air escape (breathy voice).
    Breathy,
    /// Produced with the vocal cords tightly held together, creating a crackling sound (creaky voice).
    Creaky,
    /// Standard modal voicing with regular vocal fold vibration (e.g., 'b', 'd', 'g').
    Voice,
    /// Continuant speech sounds produced with a relatively open vocal tract (e.g., nasals, liquids).
    Sonorant,
    /// Open vocal tract sounds acting as a syllable nucleus.
    Vowel,
}

/// A trait for types that can be converted **infallibly** into a [`Sound`] (doesn't consumes the input value).
///
/// ```
/// use lgg_core::Sound;
//
/// let a = Sound::vowel('a');
///
/// assert_eq!(a, "a");
/// ```
pub trait AsBytesForSound {
    fn as_bytes_for_sound(&self) -> [u8; 6];
}

/// A trait for types that can be converted **optionally** into a [`Sound`] (doesn't consumes the input value).
///
/// If the value is too large, then [`TryAsSoundError`] is returned.
///
/// ```
/// use lgg_core::sound::*;
/// 
/// // returns Ok(Sound)
/// let oue = Sound::try_vowel(['ø', 'ʊ', 'ə']);
/// 
/// // returns Err(TryAsSoundError)
/// let some_kanji = Sound::try_vowel("四叠字");
///
/// assert_eq!(oue.map(|s| s.to_string()), Ok("øʊə".to_owned()));
/// assert_eq!(some_kanji, Err(TryAsSoundError));
/// ```
pub trait TryAsBytesForSound {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError>; 
}

impl AsBytesForSound for Sound {
    fn as_bytes_for_sound(&self) -> [u8; 6] {
        self.bytes
    }
}

impl TryAsBytesForSound for Sound {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        Ok(self.bytes.clone())
    }
}

impl AsBytesForSound for char {
    fn as_bytes_for_sound(&self) -> [u8; 6] {
        let mut res = [0; 6];
        for (i, b) in self.encode_utf8(&mut [0; 4]).bytes().enumerate() {
            res[i] = b
        }
        res
    }
}

impl TryAsBytesForSound for char {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        Ok(self.as_bytes_for_sound())
    }
}

impl AsBytesForSound for [char; 1] {
    fn as_bytes_for_sound(&self) -> [u8; 6] {
        self[0].as_bytes_for_sound()
    }
}

impl TryAsBytesForSound for [char; 1] {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        Ok(self.as_bytes_for_sound())
    }
}

impl TryAsBytesForSound for str {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        (&self).try_as_bytes_for_sound()
    }
}

impl TryAsBytesForSound for &str {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        if self.len() > 6 {
            return Err(TryAsSoundError);
        }
        let mut res = [0; 6];
        for (i, b) in self.bytes().enumerate() {
            res[i] = b;
        }
        Ok(res)
    }
}

impl TryAsBytesForSound for [char] {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        (&self).try_as_bytes_for_sound()
    }
}

impl TryAsBytesForSound for &[char] {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        self.iter().collect::<String>().try_as_bytes_for_sound()
    }
}

impl TryAsBytesForSound for String {
    fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        self.as_str().try_as_bytes_for_sound()
    }
}

macro_rules! impl_try_as_bytes_for_sound_for_array {
    ($($size: literal),*) => {
        $(
            impl TryAsBytesForSound for [char; $size] {
                fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
                    self.as_slice().try_as_bytes_for_sound()
                }
            }

            impl TryAsBytesForSound for &[char; $size] {
                fn try_as_bytes_for_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
                    self.as_slice().try_as_bytes_for_sound()
                }
            }
        )*
    };
}

impl_try_as_bytes_for_sound_for_array!(2, 3, 4, 5, 6);

/// The error type returned when a value is too large to be safely allocated inside a [`Sound`].
/// 
/// ```
/// use lgg_core::sound::*;
///
/// let err = Sound::try_vowel("uuuuuuuu");
/// 
/// assert_eq!(err, Err(TryAsSoundError));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TryAsSoundError;

impl Display for TryAsSoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "too large to convert into Sound")
    }
}

impl std::error::Error for TryAsSoundError {}

#[cfg(test)]
mod sound_test {
    use super::*;

    #[test]
    fn from_char() {
        let sound = Sound::vowel('a');
        assert_eq!(sound, 'a');
        assert_eq!("a", sound);
        assert_eq!(['a'], sound);
    }

    #[test]
    fn try_from_char() {
        let sound = Sound::try_voiceless('c');
        assert_eq!(sound.map(|s| s.to_string()), Ok("c".to_string()));
    }

    #[test]
    fn try_from_str() {
        let sound = Sound::try_breathy("kʰ");
        assert_eq!(sound.map(|s| s.to_string()), Ok("kʰ".to_string()));
    }

    #[test]
    fn try_from_char_slice() {
        let sound = Sound::try_vowel(['ø', 'ʊ', 'ə']);
        assert_eq!(sound.map(|s| s.to_string()), Ok("øʊə".to_string()));
    }
}
