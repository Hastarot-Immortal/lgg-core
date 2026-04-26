use crate::Sound;

use std::{
    fmt,
    ops::{ Deref, DerefMut, Index, IndexMut },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Affix {
    sounds: Vec<Sound>,
    a_type: AffixType,
}

impl Affix {
    pub fn prefix(sounds: Vec<Sound>) -> Affix {
        Self { sounds, a_type: AffixType::Prefix }
    }

    pub fn suffix(sounds: Vec<Sound>) -> Affix {
        Self { sounds, a_type: AffixType::Suffix }
    }

    pub fn interfix(sounds: Vec<Sound>) -> Affix {
        Self { sounds, a_type: AffixType::Interfix }
    }

    pub fn affix_type(&self) -> AffixType {
        self.a_type
    }
}

impl Deref for Affix {
    type Target = Vec<Sound>;

    fn deref(&self) -> &Self::Target {
        &self.sounds
    }
}

impl DerefMut for Affix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sounds
    }
}

impl Index<usize> for Affix {
    type Output = Sound;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sounds[index]
    }
}

impl IndexMut<usize> for Affix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sounds[index]
    }
}

impl fmt::Display for Affix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.sounds
        .iter()
        .map(|sound| sound.to_string())
        .collect::<String>())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AffixType {
    Prefix,
    Suffix,
    Interfix,
}