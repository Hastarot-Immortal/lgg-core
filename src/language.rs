use crate::{Dictionary, PartOfSpeech, Rule, Word, collections::FastMap as DefaultMap};

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

/// A structural representation of a language, encapsulating an underlying vocabulary mapping system.
///
/// It acts as a wrapper around a [`Dictionary`], providing methods to build, extend, 
/// and manage a localized collection of words and their associated meanings or identifiers.
///
/// ```
/// use lgg_core::{Language, Word, PartOfSpeech, Sound};
///
/// let mut lang = Language::new();
/// let word = Word::from_slice(&[Sound::voiceless('t'), Sound::vowel('e')], PartOfSpeech::Noun);
/// 
/// lang.insert("tree".to_string(), word);
/// assert!(lang.contains(&"tree".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language<I, M=DefaultMap<I, Word>> {
	dictionary: Dictionary<I, M>,
}

impl<I> Language<I>
where
	I: Hash + Eq,
{
    /// Instantiates an empty `Language`.
    ///
    /// ```
    /// use lgg_core::Language;
    ///
    /// let lang: Language<String> = Language::new();
    /// assert!(lang.is_empty());
    /// ```
	pub fn new() -> Self {
        Self {
            dictionary: Dictionary::new(),
        }
    }

    /// Instantiates an empty `Language` pre-allocated to hold a minimum capacity of elements.
    ///
    /// ```
    /// use lgg_core::Language;
    ///
    /// let lang: Language<u32> = Language::with_capacity(100);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dictionary: Dictionary::with_capacity(capacity)
        }
    }

    /// Generates a `Language` from a fixed array of key-word entries.
    ///
    /// ```
    /// use lgg_core::{Language, Word, PartOfSpeech};
    ///
    /// let word = Word::from_array([], PartOfSpeech::Noun);
    /// let lang = Language::from_array([("hello", word)]);
    /// 
    /// assert_eq!(lang.len(), 1);
    /// ```
    pub fn from_array<const N: usize>(arr: [(I, Word); N]) -> Self {
        <Self as From<[(I, Word); N]>>::from(arr)
    }
    
    /// Generates a `Language` from an owned vector of key-word entries.
    ///
    /// ```
    /// use lgg_core::{Language, Word, PartOfSpeech};
    ///
    /// let word = Word::from_array([], PartOfSpeech::Verb);
    /// let lang = Language::from_vec(vec![(42u32, word)]);
    /// 
    /// assert_eq!(lang.len(), 1);
    /// ```
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

/// A factory trait for generating an entirely new [`Language`] instance.
/// 
/// Implementors can use this trait to define custom phonetic synthesis, generation pipelines,
/// or stateful procedural language builders.
pub trait LanguageBuilder<T, M=DefaultMap<T, Word>> {

    /// Instantiates an empty configuration state for the builder.
    fn new() -> Self;

    /// Consumes or reads the specified abstract items, running phonetic synthesis or mutation pipelines, 
    /// and constructs a fully populated [`Language`].
    ///
    /// # Examples
    ///
    /// ```
    /// use lgg_core::{Language, LanguageBuilder, PartOfSpeech};
    /// # struct MyBuilder;
    /// # impl LanguageBuilder<String> for MyBuilder {
    /// #     fn new() -> Self { MyBuilder }
    /// #     fn build<U: Into<String>, I: IntoIterator<Item=(U, PartOfSpeech)>>(&mut self, words: I) -> Language<String> { Language::new() }
    /// # }
    ///
    /// let mut builder = MyBuilder::new();
    /// let lang = builder.build([("water", PartOfSpeech::Noun)]);
    /// ```
    fn build< U: Into<T>, I: IntoIterator<Item=(U, PartOfSpeech)>>(&mut self, words: I) -> Language<T, M>;
}

/// Allows a builder to accept a chained collection of morphological [`Rule`] items.
///
/// This configuration trait is useful for attaching sequential phonological rules 
/// to a language constructor pipeline before generation.
pub trait WithRules {
    /// Consumes the builder state and updates it with the provided sequence of dynamic [`Rule`] transformers.
    fn rules<I>(self, rules: I) -> Self
    where
        I: IntoIterator<Item=Box<dyn Rule>>;
}

/// Allows a builder to accept seed values for deterministic pseudo-random generators.
pub trait WithSeed {
    /// The structural type of the random seed configuration.
	type Seed;

	/// Attaches a deterministic generation seed modifier to the builder structure.
	fn seed(self, seed: Self::Seed) -> Self;
}

/// Allows a builder to accept an explicit phonetic [`Alphabet`] specification object.
#[cfg(feature="alphabet")]
pub trait WithAlphabet {
    /// Configures the language builder framework to draw parameters from the given target alphabet.
    fn alphabet<A: Into<crate::alphabet::Alphabet>>(self, alphabet: A) -> Self;
}

/// A trait for extending an already existing [`Language`] with a set of new words.
pub trait LanguageExtender<T, M=DefaultMap<T, Word>> {
    /// Evaluates or creates words out of the raw entries iterator, appending results in-place into the target language.
    fn extend<I: IntoIterator<Item=(T, Word)>>(&mut self, language: &mut Language<T, M>, words: I);
}

/// A trait for transforming an already existing [`Language`] into a completely new `Language` object.
///
/// This can be used to simulate historical sound changes, dialectal drift, or morphological evolution 
/// mapping across vocabulary sets.
pub trait LanguageTransformer<I, M=DefaultMap<I, Word>, T=I, N=M> {
    /// Evaluates the existing dictionary framework rules to map out and output an entirely updated language generation.
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
