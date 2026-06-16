//! # lgg_core
//!
//! `lgg_core` is a structural linguistic engine and phonetic framework designed for 
//! building programmatic dictionary systems, simulating conlangs (constructed languages), 
//! and applying deterministic sound change rules.
//!
//! ## Core Architecture
//!
//! The framework is structured into several interconnected modules:
//! - [`sound`]: Defines the fundamental phoneme units ([`Sound`]) and their acoustic voicing levels.
//! - [`word`]: Handles vector sequences of sounds with attached grammatical classifications ([`Word`], [`PartOfSpeech`]).
//! - [`dictionary`]: Provides efficient unique-key lookup collections ([`Dictionary`]) to store synthesized words.
//! - [`language`]: Wraps vocabulary storage and provides builders/extenders to manage language state.
//! - [`mod@rule`]: Implements stateless and stateful rule-based transformers ([`Rule`]) to mutate words in-place.
//!
//! ## Features
//!
//! - `alphabet`: Offers an optional ordered phonetic framework pool to index, filter, and validate sounds.
//! 
//! ## Example
//! 
//! ```
//! use lgg_core::*;
//! use lgg_core::language::*;
//! 
//! struct CustomLB {
//! 	rules: Vec<Box<dyn Rule>>,
//! 	rng: SyllableRng
//! }
//! 
//! impl CustomLB {
//! 	fn new() -> Self {
//! 		Self {
//! 			rules: vec![],
//! 			rng: SyllableRng::from_seed(1),
//! 		}
//! 	}
//! }
//! 
//! impl LanguageBuilder<String> for CustomLB {
//! 	fn build< U: Into<String>, I: IntoIterator<Item=(U, PartOfSpeech)>>(&mut self, words: I) -> Language<String> {
//! 		let mut language = Language::new();
//! 
//! 		for (meaning, pos) in words {
//! 			let sounds: Vec<Sound> = vec![self.rng.next(), self.rng.next()]
//! 				.into_iter()
//! 				.flatten()
//! 				.collect();
//! 
//! 			let mut word = Word::from_vec(sounds, pos);
//! 
//! 			for rule in &self.rules {
//!           rule.apply(&mut word);
//!       }
//! 				
//! 			language.insert(meaning.into(), word);
//! 		}
//! 		
//! 		language
//! 	}
//! }
//! 
//! impl WithRules for CustomLB {
//! 	fn rules<I>(self, rules: I) -> Self
//!     where
//!         I: IntoIterator<Item=Box<dyn Rule>>
//!   {
//! 		Self {
//! 			rules: rules.into_iter().collect(),
//! 			..self
//! 		}
//!   }
//! }
//! 
//! impl WithSeed for CustomLB {
//! 	type Seed = u8;
//! 
//! 	fn seed(self, seed: Self::Seed) -> Self {
//! 		Self {
//! 			rng: SyllableRng::from_seed(seed),
//! 			..self
//! 		}
//! 	}
//! }
//! 
//! struct SyllableRng {
//! 	seed: u8,
//! }
//! 
//! impl SyllableRng {
//! 	fn from_seed(seed: u8) -> Self {
//! 		Self { seed: seed % 11 }
//! 	}
//! 
//! 	fn next(&mut self) -> Vec<Sound> {
//! 		let result = match self.seed {
//! 			0 => vec![Sound::vowel('a'), Sound::voiceless('t')],
//! 			1 => vec![Sound::sonorant('n'), Sound::vowel('e')],
//! 			2 => vec![Sound::sonorant('r'), Sound::vowel('u')],
//! 			3 => vec![Sound::voiceless('t'), Sound::vowel('u'), Sound::voiceless('t')],
//! 			4 => vec![Sound::voice('d'), Sound::vowel('e')],
//! 			5 => vec![Sound::sonorant('n'), Sound::vowel('a')],
//! 			6 => vec![Sound::vowel('u')],
//! 			7 => vec![Sound::voiceless('s'), Sound::vowel('e'), Sound::voice('d')],
//! 			8 => vec![Sound::voice('z'), Sound::vowel('u')],
//! 			9 => vec![Sound::voice('d'), Sound::vowel('u')],
//! 			_ => vec![Sound::vowel('a')],
//! 		};
//! 
//! 		self.seed = (self.seed + 7) % 11;
//! 		result
//! 	}
//! }
//! 
//! rule!(VerbEnding, move |word: &mut Word| if word.pos() == PartOfSpeech::Verb {
//! 		word.pop_if(|s: &mut Sound| s.voice_level() == VoiceLevel::Vowel);
//! 		word.append(&mut vec![Sound::vowel('e'), Sound::sonorant('n')]);
//! 	}
//! );
//! 
//! rule!(NounEnding, move |word: &mut Word| if word.pos() == PartOfSpeech::Noun {
//! 		word.push(Sound::sonorant('r'));
//! 	}
//! );
//! 
//! fn main() {
//! 	let rules: Vec<Box<dyn Rule>> = vec![
//! 		Box::new(VerbEnding),
//!		 Box::new(NounEnding),
//! 	];
//! 
//! 	let mut builder = CustomLB::new().seed(42).rules(rules);
//! 
//! 	let language = builder.build(vec![
//! 		("I", PartOfSpeech::Pron),
//!     ("love", PartOfSpeech::Verb),
//!     ("cheese", PartOfSpeech::Noun),
//! 	]);
//! 
//! 	for (meaning, word) in language.iter() {
//! 		println!("{}: {}", meaning, word);
//! 	}
//! }
//! ```

pub mod sound;
pub mod word;
pub mod dictionary;
pub mod language;
pub mod rule;
pub mod collections;
#[cfg(feature="alphabet")]
pub mod alphabet;

pub use sound::{Sound, VoiceLevel};
pub use word::{Word, PartOfSpeech};
pub use dictionary::{
	Dictionary,
	Words,
	WordsMut,
	DictIter,
	DictIterMut
};
pub use language::Language;
pub use rule::Rule;
