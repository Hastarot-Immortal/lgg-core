use crate::{sound::VowelSymbol, Sound};

pub type AlphabetKey = [char; 3];

pub trait AsKey {
    fn as_key(&self) -> AlphabetKey;
}

impl AsKey for Sound {
    fn as_key(&self) -> AlphabetKey {
        match *self {
            Sound::Consonant(ch, _) => ch.as_key(),
            Sound::Vowel(vowel_symbol) => match vowel_symbol {
                VowelSymbol::Monophthong(ch) => ch.as_key(),
                VowelSymbol::Diphthong(ch) => ch.as_key(),
                VowelSymbol::Triphthong(ch) => ch.as_key(),
            },
        }
    }
}

impl AsKey for char {
    fn as_key(&self) -> AlphabetKey {
        [*self, '\0', '\0']
    }
}

impl AsKey for [char; 2] {
    fn as_key(&self) -> AlphabetKey {
        [self[0], self[1], '\0']
    }
}

impl AsKey for [char; 3] {
    fn as_key(&self) -> AlphabetKey {
        *self
    }
}

impl AsKey for &str {
    fn as_key(&self) -> AlphabetKey {
        let mut res = ['\0'; 3];
        let iter = self.chars().take(3).enumerate();
        for (i, c) in iter {
            res[i] = c;
        }
        res
    }
}
