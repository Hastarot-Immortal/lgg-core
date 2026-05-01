use crate::Sound;
use std::{
    fmt,
    ops::{ Deref, DerefMut, Index, IndexMut },
    cmp::PartialEq,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word {
    sounds: Vec<Sound>,
    pos: PartOfSpeech,
}

impl Word {
    pub fn from_vec(sounds: Vec<Sound>, pos: PartOfSpeech) -> Self {
        Self { sounds, pos }
    }

    pub fn from_slice(sounds: &[Sound], pos: PartOfSpeech) -> Self {
        Self {
            sounds: Vec::from(sounds),
            pos,
        }
    }

    pub fn from_array<const N: usize>(sounds: [Sound; N], pos: PartOfSpeech) -> Self {
        Self { 
            sounds: Vec::from(sounds),
            pos,
        }
    }

    pub fn pos(&self) -> PartOfSpeech {
        self.pos
    }

    pub fn distance(&self, other: &Word) -> usize {
        utils::minimal_edit_distance(&self, &other, utils::DefaultCost)
    }

    pub fn distance_with_cost(&self, other: &Word, cost: impl OperatorCost) -> usize {
        utils::minimal_edit_distance(self, other, cost)
    }

    pub fn replace<F, T>(&self, from: &F, to: &T) -> Word 
    where
        F: PartialEq<Sound>,
        T: Into<Sound> + Clone,
    {
        let mut res = self.clone();
        res.replace_in_place(from, to);
        res
    }

    pub fn replacen<F, T>(&self, from: &F, to: &T, count: usize) -> Word 
    where
        F: PartialEq<Sound>,
        T: Into<Sound> + Clone,
    {
        let mut res = self.clone();
        res.replacen_in_place(from, to, count);
        res
    }

    pub fn replace_in_place<F, T>(&mut self, from: &F, to: &T) 
    where
        F: PartialEq<Sound>,
        T: Into<Sound> + Clone,
    {
        for sound in self.iter_mut() {
            if from == sound {
                *sound = to.clone().into();
            }
        }
    }

    pub fn replacen_in_place<F, T>(&mut self, from: &F, to: &T, count: usize) 
    where
        F: PartialEq<Sound>,
        T: Into<Sound> + Clone,
    {
        let mut i = 0;
        for sound in self.iter_mut() {
            if i == count {
                break;
            }
            if from == sound{
                *sound = to.clone().into();
                i += 1;
            }
        }
    }
}

impl From<(Vec<Sound>, PartOfSpeech)> for Word {
    fn from(word: (Vec<Sound>, PartOfSpeech)) -> Self {
        Self::from_vec(word.0, word.1)
    }
}

impl From<(&[Sound], PartOfSpeech)> for Word {
    fn from(word: (&[Sound], PartOfSpeech)) -> Self {
        Self::from_slice(word.0, word.1)
    }
}

impl<const N: usize> From<([Sound; N], PartOfSpeech)> for Word {
    fn from(word: ([Sound; N], PartOfSpeech)) -> Self {
        Self::from_array(word.0, word.1)
    }
}

impl Deref for Word {
    type Target = Vec<Sound>;

    fn deref(&self) -> &Self::Target {
        &self.sounds
    }
}

impl DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sounds
    }
}

impl Index<usize> for Word {
    type Output = Sound;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sounds[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sounds[index]
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.sounds
        .iter()
        .map(|sound| sound.to_string())
        .collect::<String>())
    }
}

impl PartialEq<String> for Word {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(&self.to_string(), other)
    }
}

impl PartialEq<Word> for String {
    fn eq(&self, other: &Word) -> bool {
        PartialEq::eq(&other, &self)
    }
}

impl PartialEq<&str> for Word {
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(&self.to_string()[..], &other[..])
    }
}

impl PartialEq<Word> for &str {
    fn eq(&self, other: &Word) -> bool {
        PartialEq::eq(&other, &self)
    }
}

impl PartialEq<str> for Word {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&self.to_string()[..], &other[..])
    }
}

impl PartialEq<Word> for str {
    fn eq(&self, other: &Word) -> bool {
        PartialEq::eq(&other, &self)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartOfSpeech {
    NOUN,
    VERB,
    ADJ,
    ADV,
    PRON,
    DET,
    ADP,
    NUM,
    CONJ,
    PART
}

pub trait OperatorCost: Copy + Clone {
    fn insert(&self) -> usize { 1 }
    fn delete(&self) -> usize { 1 }
    fn substitution(&self, first: &Sound, second: &Sound) -> usize {
        if first == second { 0 } else { 1 }
    }
}

mod utils {
    use super::*;

    #[derive(Clone, Copy)]
    pub(super) struct DefaultCost;

    impl OperatorCost for DefaultCost {}

    pub(super) fn minimal_edit_distance(
        first: &Word,
        second: &Word,
        cost: impl OperatorCost,
    ) -> usize {
        let first_len = first.len();
        let second_len = second.len();

        let mut matrix = vec![vec![0usize; first_len + 1]; second_len + 1];

        for i in 1..=second_len {
            matrix[i][0] = matrix[i - 1][0] + cost.insert();
        }

        for i in 1..=first_len {
            matrix[0][i] = matrix[0][i - 1] + cost.delete();
        }

        for i in 1..=second_len {
            for j in 1..=first_len {
                matrix[i][j] = min(
                    matrix[i - 1][j] + cost.delete(),
                    matrix[i - 1][j - 1] + cost.substitution(&first[j - 1], &second[i - 1]),
                    matrix[i][j - 1] + cost.insert(),
                );
            }
        }

        matrix[second_len][first_len]
    }

    fn min(first: usize, second: usize, third: usize) -> usize {
        first.min(second).min(third)
    }
}

#[cfg(test)]
mod word_test {
    use super::*;
    use crate::VoiceLevel;

    #[test]
    fn replace() {
        let t = Sound::new('t', VoiceLevel::Voiceless);
        let e = Sound::monophthong('e');
        let a = Sound::monophthong('a');
        let w = Sound::new('w', VoiceLevel::Sonorant);
        let z = Sound::new('z', VoiceLevel::Voice);

        let word1 = Word::from(([t, e, e, w, a, z], PartOfSpeech::NOUN));
        let word2 = word1.clone();

        let mut word3 = word1.replace(&e, &a);
        let mut word4 = word2.replacen(&e, &a, 1);
    
        word3.replace_in_place(&'a', &('e', VoiceLevel::Vowel));
        word4.replacen_in_place(&"a", &e, 1);

        assert_eq!(word3, "teewez");
        assert_eq!(word4, word2);
    }
}