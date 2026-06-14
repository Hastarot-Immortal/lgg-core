use crate::{Dictionary, PartOfSpeech, Rule, Word, collections::FastMap as DefaultMap};

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language<I, M=DefaultMap<I, Word>> {
	dictionary: Dictionary<I, M>,
}

impl<I> Language<I>
where
	I: Hash + Eq,
{
	pub fn new() -> Self {
        Self {
            dictionary: Dictionary::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dictionary: Dictionary::with_capacity(capacity)
        }
    }

    pub fn from_array<const N: usize>(arr: [(I, Word); N]) -> Self {
        <Self as From<[(I, Word); N]>>::from(arr)
    }

    pub fn from_vec(vec: Vec<(I, Word)>) -> Self {
        Self {
            dictionary: Dictionary::from_iter(vec),
        }
    }
}

impl<I, M> From<Dictionary<I, M>> for Language<I, M> {
    fn from(dictionary: Dictionary<I, M>) -> Self {
        Self { dictionary }
    }
}

impl<I, M, const N: usize> From<[(I, Word); N]> for Language<I, M>
where
    M: From<[(I, Word); N]>,
{
    fn from(arr: [(I, Word); N]) -> Self {
        Self {
            dictionary: Dictionary::from(arr),
        }
    }
}

impl<I, M> FromIterator<(I, Word)> for Language<I, M>
where
    M: FromIterator<(I, Word)>,
{
    fn from_iter<T: IntoIterator<Item = (I, Word)>>(iter: T) -> Language<I, M> {
        Self {
            dictionary: Dictionary::from_iter(iter),
        }
    }
}

impl<I, M> Deref for Language<I, M> {
    type Target = Dictionary<I, M>;

    fn deref(&self) -> &Self::Target {
        &self.dictionary
    }
}

impl<I, M> DerefMut for Language<I, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dictionary
    }
}

pub trait LanguageBuilder<T, M=DefaultMap<T, Word>> {
    fn new() -> Self;
    fn build< U: Into<T>, I: IntoIterator<Item=(U, PartOfSpeech)>>(&mut self, words: I) -> Language<T, M>;
}

pub trait WithRules {
    fn rules<I>(self, rules: I) -> Self
    where
        I: IntoIterator<Item=Box<dyn Rule>>;
}

pub trait WithSeed {
	type Seed;
	fn seed(self, seed: Self::Seed) -> Self;
}

#[cfg(feature="alphabet")]
pub trait WithAlphabet {
    fn alphabet<A: Into<crate::alphabet::Alphabet>>(self, alphabet: A) -> Self;
}

pub trait LanguageExtender<T, M=DefaultMap<T, Word>> {
    fn extend<I: IntoIterator<Item=(T, Word)>>(&mut self, language: &mut Language<T, M>, words: I);
}

pub trait LanguageTransformer<I, M=DefaultMap<I, Word>, T=I, N=M> {
    fn transform(&mut self, language: &Language<I, M>) -> Language<T, N>;
}

#[cfg(test)]
mod language_test {
    use super::*;
    use crate::{Sound, rule, Rule};

    fn create_simple_alphabet() -> (Sound, Sound, Sound, Sound) {
        (
            Sound::voiceless('t'),
            Sound::vowel('e'),
            Sound::sonorant('w'),
            Sound::voice('z')
        )
    }

    #[test]
    fn insert() {
        let (t, e, w, z) = create_simple_alphabet();

        let mut language = Language::with_capacity(3);
        language.insert(1, Word::from_slice(&[t, e, w], PartOfSpeech::Noun));
        language.insert(2, Word::from_array([z, e, w], PartOfSpeech::Verb));
        language.insert(3, Word::from_vec(vec![t, e, z], PartOfSpeech::Pron));

        assert_eq!(language.get(&1).map(|word| word.to_string()), Some("tew".to_string()));
        assert_eq!(language.get(&2).map(|word| word.to_string()), Some("zew".to_string()));
        assert_eq!(language.get(&3).map(|word| word.to_string()), Some("tez".to_string()));
    }

    #[derive(Default)]
    struct MockRng(u8);

    impl MockRng {
        fn next_sound(&mut self) -> Sound {
            let res = match self.0 {
                0 => Sound::voiceless('t'),
                1 => Sound::vowel('e'),
                2 => Sound::sonorant('w'),
                3 => Sound::voice('z'),
                4 => Sound::sonorant('n'),
                5 => Sound::voice('v'),
                _ => Sound::voiceless('s'),
            };
            self.0 += 1;
            if self.0 > 6 {
                self.0 = 0;
            }
            res
        }
    }

    struct TestLB {
        rules: Vec<Box<dyn Rule>>,
        rng: MockRng,
    }

    impl LanguageBuilder<String> for TestLB {
        fn new() -> Self {
            Self {
                rules: vec![],
                rng: MockRng(0),
            }
        }

        fn build< U: Into<String>, I: IntoIterator<Item=(U, PartOfSpeech)>>(&mut self, words: I) -> Language<String> {
            let mut language = Language::new();

            for (meaning, pos) in words {
                let mut word = Word::from({
                    let mut sounds = Vec::with_capacity(4);
                    for _ in 0..4 {
                        sounds.push(self.rng.next_sound());
                    }
                    (sounds, pos)
                });
                for rule in &self.rules {
                    rule.apply(&mut word);
                }
                language.insert(meaning.into(), word);
            }

            language
        }
    }

    impl WithRules for TestLB {
        fn rules<I>(self, rules: I) -> Self
        where
            I: IntoIterator<Item=Box<dyn Rule>>
        {
            Self {
                rules: Vec::from_iter(rules),
                ..self
            }
        }
    }

    impl WithSeed for TestLB {
        type Seed = u8;
        fn seed(self, seed: Self::Seed) -> Self {
            Self {
                rng: MockRng(seed % 7),
                ..self
            }
        }
    }

    rule!(DoubleNEnding, |word: &mut Word| if word.last().is_some_and(|s| *s == 'n') {
        word.push(Sound::sonorant('n'));
    });

    rule!(VerbEnding, |word: &mut Word| if word.pos() == PartOfSpeech::Verb {
        word.push(Sound::vowel('e'));
        word.push(Sound::sonorant('n'));
    });

    #[test]
    fn test_lb() {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(DoubleNEnding),
            Box::new(VerbEnding),
        ];

        let mut builder = TestLB::new().rules(rules).seed(1);
        let language = builder.build([
            ("I", PartOfSpeech::Pron),
            ("love", PartOfSpeech::Verb),
            ("you", PartOfSpeech::Pron),
        ]);
        assert_eq!(language.get(&"I".to_string()).map(|w| w.to_string()), Some("ewznn".to_string()));
        assert_eq!(language.get(&"love".to_string()).map(|w| w.to_string()), Some("vsteen".to_string()));
        assert_eq!(language.get(&"you".to_string()).map(|w| w.to_string()), Some("wznv".to_string()));
    }
}
