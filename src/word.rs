use crate::Sound;
use std::{
    fmt,
    ops::{Deref, DerefMut, Index, IndexMut},
    cmp::PartialEq,
};

/// Represents word as a vector of [`Sound`] with its grammatical type [`PartOfSpeech`]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word {
    sounds: Vec<Sound>,
    pos: PartOfSpeech,
}

impl Word {
    /// Instantiates a word from a `Vec` of [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let sounds = vec![
    ///     Sound::voiceless('s'),
    ///     Sound::vowel('e'),
    ///     Sound::vowel('a'),
    /// ];
    ///
    /// let word = Word::from_vec(sounds, PartOfSpeech::Noun);
    ///
    /// assert_eq!(word, "sea");
    /// ```
    pub fn from_vec(sounds: Vec<Sound>, pos: PartOfSpeech) -> Self {
        Self { sounds, pos }
    }

    /// Instantiates a word from a slice of [`Sound`].
    ///
    ///```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let word = Word::from_slice([
    ///     Sound::voiceless('f'),
    ///     Sound::vowel('e'),
    ///     Sound::vowel('a'),
    ///     Sound::sonorant('r'),
    /// ].as_slice(), PartOfSpeech::Noun);
    ///
    /// assert_eq!(word, "fear");
    ///```
    pub fn from_slice(sounds: &[Sound], pos: PartOfSpeech) -> Self {
        Self {
            sounds: Vec::from(sounds),
            pos,
        }
    }
    
    /// Instantiates a word stack-allocated array of [`Sound`].
    ///
    ///```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let word = Word::from_array([
    ///     Sound::voiceless('t'),
    ///     Sound::vowel('e'),
    ///     Sound::vowel('a'),
    /// ], PartOfSpeech::Noun);
    ///
    /// assert_eq!(word, "tea");
    ///```
    pub fn from_array<const N: usize>(sounds: [Sound; N], pos: PartOfSpeech) -> Self {
        Self { 
            sounds: Vec::from(sounds),
            pos,
        }
    }
    
    /// Returns the [`PartOfSpeech`] tag of the word.
    pub fn pos(&self) -> PartOfSpeech {
        self.pos
    }

    /// Calculates the Damerau–Levenshtein minimal edit distance between this word and another one using standard uniform costs.
    ///
    /// The default implementation weights standard structural mutations (insertions, deletions, transpositions, substitutions) as `1`.
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let t = Sound::voiceless('t');
    /// let e = Sound::vowel('e');
    /// let a = Sound::vowel('a');
    /// let w = Sound::sonorant('w');
    ///
    /// let word1 = Word::from(([t, e, a], PartOfSpeech::Noun));
    /// let word2 = Word::from(([t, a, w], PartOfSpeech::Noun));
    ///
    /// assert_eq!(word1.distance(&word2), 2);
    /// ```
    pub fn distance(&self, other: &Word) -> usize {
        distance::minimal_edit_distance(self, &other, &distance::DefaultCost)
    }

    /// Calculates the Damerau–Levenshtein minimal edit distance between this word and another one using custom algorithmic weights.
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, word::DistanceCost};
    ///
    /// pub struct MyDistanceCost;
    ///
    /// impl DistanceCost for MyDistanceCost {
    ///     fn transposition(&self) -> usize { 2 }
    /// }
    ///
    /// let s = Sound::voiceless('s');
    /// let e = Sound::vowel('e');
    /// let a = Sound::vowel('a');
    ///
    /// let word1 = Word::from(([s, e, a], PartOfSpeech::Noun));
    /// let word2 = Word::from(([s, a, e], PartOfSpeech::Adj));
    ///
    /// assert_eq!(word1.distance_with_cost(&word2, &MyDistanceCost), 2);
    /// ```
    pub fn distance_with_cost(&self, other: &Word, cost: &impl DistanceCost) -> usize {
        distance::minimal_edit_distance(self, other, cost)
    }

    /// Calculates the minimal edit distance between this word and another one using custom algorithm.
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, word::levenshtein_distance};
    ///
    /// let w = Sound::sonorant('w');
    /// let o = Sound::vowel('o');
    /// let a = Sound::vowel('a');
    ///
    /// let word1 = Word::from(([w, o, a, w], PartOfSpeech::Noun));
    /// let word2 = Word::from(([w, a, o, o], PartOfSpeech::Noun));
    ///
    /// assert_eq!(word1.distance_with(&word2, levenshtein_distance), 3);
    /// assert_eq!(word1.distance(&word2), 2);
    /// ```
    pub fn distance_with<F: FnOnce(&Word, &Word) -> usize>(&self, other: &Word, f: F) -> usize {
        f(self, other)
    }

    /// Calculates the minimal edit distance between this word and another one using custom algorithm and weights.
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, word::{DistanceCost, levenshtein_distance_with_cost}};
    ///
    /// pub struct DoubleDistanceCost;
    ///
    /// impl DistanceCost for DoubleDistanceCost {
    ///     fn insert(&self) -> usize { 2 }
    ///     fn delete(&self) -> usize { 2 }
    ///     fn transposition(&self) -> usize { 2 }
    ///     fn substitution(&self) -> usize { 2 }
    /// }
    ///
    /// let w = Sound::sonorant('w');
    /// let o = Sound::vowel('o');
    /// let a = Sound::vowel('a');
    ///
    /// let word1 = Word::from(([w, o, a, w], PartOfSpeech::Noun));
    /// let word2 = Word::from(([w, a, o, o], PartOfSpeech::Noun));
    ///
    /// assert_eq!(word1.distance_with_fn_and_cost(&word2, levenshtein_distance_with_cost, &DoubleDistanceCost), 6);
    /// assert_eq!(word1.distance(&word2), 2);
    /// ```
    pub fn distance_with_fn_and_cost<C: DistanceCost, F: FnOnce(&Word, &Word, &C) -> usize>(&self, other: &Word, f: F, cost: &C) -> usize {
        f(self, other, cost)
    }

    /// Creates a new `Word`, replacing all matches of a pattern with another [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let s = Sound::voiceless('s');
    /// let a = Sound::vowel('a');
    /// let z = Sound::voice('z');
    /// let w = Sound::sonorant('w');
    ///
    /// let word = Word::from(([s, a, w, s, a], PartOfSpeech::Noun));
    ///
    /// let new_word = word.replace(&s, &z);
    ///
    /// assert_eq!(new_word, "zawza");
    /// ```
    pub fn replace<F, T>(&self, from: &F, to: &T) -> Word 
    where
        F: PartialEq<Sound>,
        T: Into<Sound> + Clone,
    {
        let mut res = self.clone();
        res.replace_in(from, to);
        res
    }

    /// Creates a new `Word`, replacing first N matches of a pattern with another [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let t = Sound::voiceless('t');
    /// let a = Sound::vowel('a');
    /// let d = Sound::voice('d');
    ///
    /// let word = Word::from(([t, a, t, t, a, t], PartOfSpeech::Noun));
    ///
    /// let new_word = word.replacen(&t, &d, 2);
    ///
    /// assert_eq!(new_word, "dadtat");
    /// ```
    pub fn replacen<F, T>(&self, from: &F, to: &T, count: usize) -> Word 
    where
        F: PartialEq<Sound>,
        T: Into<Sound> + Clone,
    {
        let mut res = self.clone();
        res.replacen_in(from, to, count);
        res
    }

    /// Replaces all matches of a pattern with another [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let t = Sound::voiceless('t');
    /// let a = Sound::vowel('a');
    /// let s = Sound::voice('s');
    ///
    /// let mut word = Word::from(([t, a, s, t], PartOfSpeech::Pron));
    ///
    /// word.replace_in(&t, &s);
    ///
    /// assert_eq!(word, "sass");
    /// ```
    pub fn replace_in<F, T>(&mut self, from: &F, to: &T) 
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

    /// Replaces first N matches of a pattern with another [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound};
    ///
    /// let t = Sound::voiceless('t');
    /// let a = Sound::vowel('a');
    /// let s = Sound::voice('s');
    ///
    /// let mut word = Word::from(([t, a, s, t], PartOfSpeech::Pron));
    ///
    /// word.replacen_in(&t, &s, 1);
    ///
    /// assert_eq!(word, "sast");
    /// ```
    pub fn replacen_in<F, T>(&mut self, from: &F, to: &T, count: usize) 
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

    /// Attempts to create a new `Word`, replacing all matches of a pattern with another [`Sound`].
    ///
    /// # Errors
    /// Returns the matching type error if the target symbol cannot successfully parse into a [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, VoiceLevel};
    ///
    /// let s = Sound::voiceless('s');
    /// let a = Sound::vowel('a');
    /// let z = Sound::voice('z');
    ///
    /// let word = Word::from(([s, a, z, a], PartOfSpeech::Noun));
    ///
    /// let new_word = word.try_replace(&a, &("ea", VoiceLevel::Vowel));
    ///
    /// assert_eq!(new_word.map(|w| w.to_string()), Ok("seazea".to_string()));
    /// ```
    pub fn try_replace<F, T>(&self, from: &F, to: &T) -> Result<Word, <T as TryInto<Sound>>::Error>
    where
        F: PartialEq<Sound>,
        T: TryInto<Sound> + Clone,
    {
        Ok(self.replace(from, &to.clone().try_into()?))
    }

    /// Attempts to create a new `Word`, replacing first N matches of a pattern with another [`Sound`].
    ///
    /// # Errors
    /// Returns the matching type error if the target symbol cannot successfully parse into a [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, VoiceLevel};
    ///
    /// let a = Sound::vowel('a');
    /// let z = Sound::voice('z');
    ///
    /// let word = Word::from(([z, a, z, a], PartOfSpeech::Noun));
    ///
    /// let new_word = word.try_replacen(&a, &("ea", VoiceLevel::Vowel), 1);
    ///
    /// assert_eq!(new_word.map(|w| w.to_string()), Ok("zeaza".to_string()));
    /// ```
    pub fn try_replacen<F, T>(&self, from: &F, to: &T, count: usize) -> Result<Word, <T as TryInto<Sound>>::Error> 
    where
        F: PartialEq<Sound>,
        T: TryInto<Sound> + Clone,
    {
        Ok(self.replacen(from, &to.clone().try_into()?, count))
    }

    /// Attempts to replace all matches of a pattern with another [`Sound`].
    ///
    /// # Errors
    /// Returns the matching type error if the target symbol cannot successfully parse into a [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, VoiceLevel};
    ///
    /// let t = Sound::voiceless('t');
    /// let a = Sound::vowel('a');
    /// let d = Sound::voice('d');
    ///
    /// let mut word = Word::from(([t, a, d, a], PartOfSpeech::Noun));
    ///
    /// word.try_replace_in(&a, &("ea", VoiceLevel::Vowel));
    ///
    /// assert_eq!(word, "teadea");
    /// ```
    pub fn try_replace_in<F, T>(&mut self, from: &F, to: &T) -> Result<(), <T as TryInto<Sound>>::Error>
    where
        F: PartialEq<Sound>,
        T: TryInto<Sound> + Clone,
    {
        to.clone().try_into().map(|new_sound| {
            self.replace_in(from, &new_sound);
            ()
        })
    }

    /// Attempts to replace first N matches of a pattern with another [`Sound`].
    ///
    /// # Errors
    /// Returns the matching type error if the target symbol cannot successfully parse into a [`Sound`].
    ///
    /// ```
    /// use lgg_core::{Word, PartOfSpeech, Sound, VoiceLevel};
    ///
    /// let t = Sound::voiceless('t');
    /// let a = Sound::vowel('a');
    ///
    /// let mut word = Word::from(([t, a, t, a], PartOfSpeech::Noun));
    ///
    /// word.try_replacen_in(&a, &("ou", VoiceLevel::Vowel), 1);
    ///
    /// assert_eq!(word, "touta");
    /// ```
    pub fn try_replacen_in<F, T>(&mut self, from: &F, to: &T, count: usize) -> Result<(), <T as TryInto<Sound>>::Error>
    where
        F: PartialEq<Sound>,
        T: TryInto<Sound> + Clone,
    {
        to.clone().try_into().map(|new_sound| {
            self.replacen_in(from, &new_sound, count);
            ()
        })
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

macro_rules! impl_partial_eq {
    ($T:ty) => {
        impl PartialEq<$T> for Word {
            fn eq(&self, other: &$T) -> bool {
                PartialEq::eq(&self.to_string()[..], &other[..])
            }
        }

        impl PartialEq<Word> for $T {
            fn eq(&self, other: &Word) -> bool {
                PartialEq::eq(&other, &self)
            }
        }
    };
}

impl_partial_eq!(String);
impl_partial_eq!(str);
impl_partial_eq!(&str);

/// A grammatical category or [`Word`]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartOfSpeech {
    /// Noun (substantive objects/concepts)
    Noun,
    /// Verb (action or state states)
    Verb,
    /// Adjective (modifiers of nouns)
    Adj,
    /// Adverb (modifiers of verbs, adjectives, or other adverbs)
    Adv,
    /// Pronoun (structural replacement markers)
    Pron,
    /// Determiner (identifying context markers)
    Det,
    /// Adposition (prepositions and postpositions)
    Adp,
    /// Numeral (cardinal or ordinal representations)
    Num,
    /// Conjunction (syntactic coordinators)
    Conj,
    /// Particle (independent grammatical markers)
    Part
}

/// A metric configuration trait defining penalties for minimal edit distance calculations.
pub trait DistanceCost {
    /// Penalty cost when inserting a new `Sound`. Defaults to `1`.
    fn insert(&self) -> usize { 1 }
    
    /// Penalty cost when deleting an existing `Sound`. Defaults to `1`.
    fn delete(&self) -> usize { 1 }

    /// Penalty cost when swaping two successive `Sound`s. Defaults to `1`.
    fn transposition(&self) -> usize { 1 }
    
    /// Penalty cost when converting one `Sound` to another. Defaults to `1`.
    fn substitution(&self) -> usize { 1 }
}

/// Function to calculate the Levenshtein minimum edit distance between this word and another one using custom weights.
pub fn levenshtein_distance_with_cost(first: &Word, second: &Word, cost: &impl DistanceCost) -> usize {
    let (first_len, second_len, mut matrix) = distance::prepare_matrix(first, second, cost);

    for i in 1..=second_len {
        for j in 1..=first_len {
            matrix[i][j] = distance::min(
                matrix[i - 1][j] + cost.delete(),
                matrix[i - 1][j - 1] + if first[j - 1] == second[i - 1] { 0 } else { cost.substitution() },
                matrix[i][j - 1] + cost.insert(),
            );
        }
    }
        
    matrix[second_len][first_len]
}

/// Function to calculate the Levenshtein minimum edit distance between this word and another one using standard uniform weights.
///
/// The default implementation weights standard structural mutations (insertions, deletions, substitutions) as `1`.
pub fn levenshtein_distance(first: &Word, second: &Word) -> usize {
    levenshtein_distance_with_cost(first, second, &distance::DefaultCost)
}

mod distance {
    use super::*;

    #[derive(Clone, Copy)]
    pub(super) struct DefaultCost;

    impl DistanceCost for DefaultCost {}

    pub(super) fn minimal_edit_distance(
        first: &Word,
        second: &Word,
        cost: &impl DistanceCost,
    ) -> usize {
        let (first_len, second_len, mut matrix) = prepare_matrix(first, second, cost);

        for i in 1..=second_len {
            for j in 1..=first_len {
                matrix[i][j] = min(
                    matrix[i - 1][j] + cost.delete(),
                    matrix[i - 1][j - 1] + if first[j - 1] == second[i - 1] { 0 } else { cost.substitution() },
                    matrix[i][j - 1] + cost.insert(),
                );
                if i > 1 && j > 1 && (first[j - 1] == second[i - 2]) && (first[j - 2] == second[i - 1]) {
                    matrix[i][j] = matrix[i][j].min(matrix[i - 2][j - 2] + cost.transposition());
                }
            }
        }
        
        matrix[second_len][first_len]
    }

    pub(super) fn min(first: usize, second: usize, third: usize) -> usize {
        first.min(second).min(third)
    }

    pub(super) fn prepare_matrix(
        first: &Word,
        second: &Word,
        cost: &impl DistanceCost,
    ) -> (usize, usize, Vec<Vec<usize>>) {
        let first_len = first.len();
        let second_len = second.len();

        let mut matrix = vec![vec![0usize; first_len + 1]; second_len + 1];

        for i in 1..=second_len {
            matrix[i][0] = matrix[i - 1][0] + cost.insert();
        }

        for i in 1..=first_len {
            matrix[0][i] = matrix[0][i - 1] + cost.delete();
        }
        (first_len, second_len, matrix)
    }
}

#[cfg(test)]
mod word_test {
    use super::*;
    use crate::VoiceLevel;

    fn create_simple_alphabet() -> (Sound, Sound, Sound, Sound, Sound) {
        (
            Sound::voiceless('t'),
            Sound::vowel('e'),
            Sound::vowel('a'),
            Sound::sonorant('w'),
            Sound::voice('z')
        )
    }

    #[test]
    fn replace() {
        let (t, e, a, w, z) = create_simple_alphabet();

        let word1 = Word::from(([t, e, e, w, a, z], PartOfSpeech::Noun));
        let word2 = word1.clone();

        let mut word3 = word1.replace(&e, &a);
        let mut word4 = word2.replacen(&e, &a, 1);
    
        word3.replace_in(&'a', &('o', VoiceLevel::Vowel));
        word4.replacen_in(&"a", &e, 1);

        assert_eq!(word3, "toowoz");
        assert_eq!(word4, word2);
    }

    fn hamming_distance(first: &Word, second: &Word) -> usize {
        first.iter()
        .zip(second.iter())
        .fold(0usize, |acc, (f, s)| acc + if f != s { 1 } else { 0 }) 
    }

    #[test]
    fn distance() {
        let (t, e, a, w, z) = create_simple_alphabet();

        let word1 = Word::from(([t, e, e, w, a, z], PartOfSpeech::Noun));
        let word2 = Word::from(([t, e, a, w, z], PartOfSpeech::Noun));
        let dist = word1.distance(&word2);
        assert_eq!(dist, 2);

        let word1 = Word::from(([t, e, a, w, a, z], PartOfSpeech::Noun));
        let word2 = Word::from(([t, a, e, w, a, z], PartOfSpeech::Noun));
        let dist = word1.distance(&word2);
        assert_eq!(dist, 1);

        let word1 = Word::from(([t, e, a, w, a, z], PartOfSpeech::Noun));
        let word2 = Word::from(([t, a, e, w, a, z], PartOfSpeech::Noun));
        let dist = word1.distance_with(&word2, hamming_distance);
        assert_eq!(dist, 2);
    }
}
