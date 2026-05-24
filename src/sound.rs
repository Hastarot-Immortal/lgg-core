use std::fmt::{Display, Formatter, Error as FmtError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sound {
    bytes: [u8; 6],
    level: VoiceLevel,
}

impl Sound {
    pub fn from<I: AsSound>(symbol: I, level: VoiceLevel) -> Self {
        Self {
            bytes: symbol.as_sound(),
            level,
        }
    }

    pub fn try_from<I: TryAsSound>(symbol: I, level: VoiceLevel) -> Result<Self, TryAsSoundError> {
        Ok(Self {
            bytes: symbol.try_as_sound()?,
            level,
        })
    }

    pub fn vowel<I: AsSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Vowel)
    }

    pub fn try_vowel<I: TryAsSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Vowel)
    }

    pub fn sonorant<I: AsSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Sonorant)
    }

    pub fn try_sonorant<I: TryAsSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Sonorant)
    }

    pub fn voice<I: AsSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Voice)
    }

    pub fn try_voice<I: TryAsSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Voice)
    }

    pub fn creaky<I: AsSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Creaky)
    }

    pub fn try_creaky<I: TryAsSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Creaky)
    }

    pub fn breathy<I: AsSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Breathy)
    }

    pub fn try_breathy<I: TryAsSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Breathy)
    }

    pub fn voiceless<I: AsSound>(symbol: I) -> Self {
        Self::from(symbol, VoiceLevel::Voiceless)
    }

    pub fn try_voiceless<I: TryAsSound>(symbol: I) -> Result<Self, TryAsSoundError> {
        Self::try_from(symbol, VoiceLevel::Voiceless)
    }

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
                other.try_as_sound().is_ok_and(|bytes| bytes == self.bytes)
            }
        }

        impl PartialEq<&$other> for Sound {
            fn eq(&self, other: &&$other) -> bool {
                PartialEq::eq(self, *other)
            }
        }

        impl PartialEq<Sound> for $other {
            fn eq(&self, other: &Sound) -> bool {
                self.try_as_sound().is_ok_and(|bytes| bytes == other.bytes)
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VoiceLevel {
    Voiceless,
    Breathy,
    Creaky,
    Voice,
    Sonorant,
    Vowel,
}

pub trait AsSound {
    fn as_sound(&self) -> [u8; 6];
}

pub trait TryAsSound {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError>; 
}

impl AsSound for Sound {
    fn as_sound(&self) -> [u8; 6] {
        self.bytes
    }
}

impl TryAsSound for Sound {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        Ok(self.bytes.clone())
    }
}

impl AsSound for char {
    fn as_sound(&self) -> [u8; 6] {
        let mut res = [0; 6];
        for (i, b) in self.encode_utf8(&mut [0; 4]).bytes().enumerate() {
            res[i] = b
        }
        res
    }
}

impl TryAsSound for char {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        Ok(self.as_sound())
    }
}

impl TryAsSound for str {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        (&self).try_as_sound()
    }
}

impl TryAsSound for &str {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
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

impl TryAsSound for [char] {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        (&self).try_as_sound()
    }
}

impl TryAsSound for &[char] {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        self.iter().collect::<String>().try_as_sound()
    }
}

impl TryAsSound for String {
    fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
        self.as_str().try_as_sound()
    }
}

macro_rules! impl_try_as_sound_for_array {
    ($($size: literal),*) => {
        $(
            impl TryAsSound for [char; $size] {
                fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
                    self.as_slice().try_as_sound()
                }
            }

            impl TryAsSound for &[char; $size] {
                fn try_as_sound(&self) -> Result<[u8; 6], TryAsSoundError> {
                    self.as_slice().try_as_sound()
                }
            }
        )*
    };
}

impl_try_as_sound_for_array!(1, 2, 3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TryAsSoundError;

impl Display for TryAsSoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "this is too big to convert into Sound")
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
