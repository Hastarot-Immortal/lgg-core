use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sound {
    Consonant(char, VoiceLevel),
    Vowel(VowelSymbol),
}

impl Sound {
    pub fn new(symbol: char, level: VoiceLevel) -> Self {
        match level {
            VoiceLevel::Vowel => Self::monophthong(symbol),
            _ => Self::Consonant(symbol, level),
        }
    }

    pub fn monophthong(symbol: char) -> Self {
        Self::Vowel(VowelSymbol::Monophthong(symbol))
    }

    pub fn diphthong(symbol: [char; 2]) -> Self {
        Self::Vowel(VowelSymbol::Diphthong(symbol))
    }

    pub fn triphthong(symbol: [char; 3]) -> Self {
        Self::Vowel(VowelSymbol::Triphthong(symbol))
    }

    pub fn voice_level(&self) -> VoiceLevel {
        match *self {
            Self::Consonant(_, level) => level,
            Self::Vowel(_) => VoiceLevel::Vowel,
        }
    }
}

impl From<(char, VoiceLevel)> for Sound {
    fn from(sound: (char, VoiceLevel)) -> Self {
        Self::new(sound.0, sound.1)
    }
}

impl From<[char; 2]> for Sound {
    fn from(sound: [char; 2]) -> Self {
        Self::diphthong(sound)
    }
}

impl From<[char; 3]> for Sound {
    fn from(sound: [char; 3]) -> Self {
        Self::triphthong(sound)
    }
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                Self::Consonant(symbol, _) => symbol.to_string(),
                Self::Vowel(symbol) => match symbol {
                    VowelSymbol::Monophthong(sym) => sym.to_string(),
                    VowelSymbol::Diphthong(sym) => {
                        let mut res = String::with_capacity(
                            sym
                            .iter()
                            .map(|s| s.len_utf8())
                            .sum());
                        res.push(sym[0]);
                        res.push(sym[1]);
                        res
                    }
                    VowelSymbol::Triphthong(sym) => {
                        let mut res = String::with_capacity(
                            sym
                            .iter()
                            .map(|s| s.len_utf8())
                            .sum());
                        res.push(sym[0]);
                        res.push(sym[1]);
                        res.push(sym[2]);
                        res
                    }
                },
            }
        )
    }
}

impl PartialEq<char> for Sound {
    fn eq(&self, other: &char) -> bool {
        match *self {
            Self::Consonant(symbol, _) => symbol == *other,
            Self::Vowel(symbol) => match symbol {
                VowelSymbol::Monophthong(sym) => sym == *other,
                VowelSymbol::Diphthong(_) => false,
                VowelSymbol::Triphthong(_) => false,
            }
        }
    }
}

impl PartialEq<Sound> for char {
    fn eq(&self, other: &Sound) -> bool {
        PartialEq::eq(other, self)
    }
}

impl PartialEq<&str> for Sound {
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(&self.to_string().as_str(), other)
    }
}

impl PartialEq<Sound> for &str {
    fn eq(&self, other: &Sound) -> bool {
        PartialEq::eq(&other.to_string().as_str(), self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VowelSymbol {
    Monophthong(char),
    Diphthong([char; 2]),
    Triphthong([char; 3]),
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

#[cfg(test)]
mod sound_test {
    use super::*;

    #[test]
    fn monophthong() {
        let s = Sound::monophthong('a');
        assert_eq!(s, 'a');
    }

    #[test]
    fn diphthong() {
        let s = Sound::diphthong(['e', 'a']);
        assert_eq!(s, "ea");
    }

    #[test]
    fn triphthong() {
        let s = Sound::triphthong(['e', 'o', 'a']);
        assert_eq!(s, "eoa");
    }
}
