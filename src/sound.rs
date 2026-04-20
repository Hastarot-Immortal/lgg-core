use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", match *self {
            Self::Consonant(symbol, _) => symbol.to_string(),
            Self::Vowel(symbol) => match symbol {
                VowelSymbol::Monophthong(sym) => sym.to_string(),
                VowelSymbol::Diphthong(sym) => {
                    let mut res = String::with_capacity(2);
                    res.push(sym[0]);
                    res.push(sym[1]);
                    res
                }
                VowelSymbol::Triphthong(sym) => {
                    let mut res = String::with_capacity(3);
                    res.push(sym[0]);
                    res.push(sym[1]);
                    res.push(sym[2]);
                    res
                }
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VowelSymbol {
    Monophthong(char),
    Diphthong([char; 2]),
    Triphthong([char; 3]),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VoiceLevel {
    Voiceless,
    Breathy,
    Creaky,
    Voice,
    Sonorant,
    Vowel,
}
