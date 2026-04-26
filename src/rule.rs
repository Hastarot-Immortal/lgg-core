use crate::Word;

pub trait Rule {
	fn apply(&self, word: &mut Word);
}

#[macro_export]
macro_rules! rule {
	($name: ident, $func: expr) => {
		pub struct $name;

		impl Rule for $name {
			fn apply(&self, word: &mut Word) {
				($func)(word)
			}
		}
	}
}

#[cfg(test)]
mod rule_test {
	use super::*;
	use crate::{ Sound, PartOfSpeech::NOUN, VoiceLevel};

	rule!(Rule1, move |word: &mut Word| word.swap(0, 1));
	
	rule!(Rule2, move |word: &mut Word| {
		for sound in word.iter_mut() {
			if *sound == 'a' {
				*sound = Sound::diphthong(['e', 'a']);
			}
		}
	});

	#[test]
	fn rule1() {
		let a = Sound::monophthong('a');
		let e = Sound::monophthong('e');

		let mut word = Word::from_slice(&[a, e], NOUN);
		let rule = Rule1;
		rule.apply(&mut word);
		assert_eq!(word, Word::from_slice(&[e, a], NOUN));
	}

	#[test]
	fn rule2() {
		let a = Sound::monophthong('a');
		let m = Sound::new('m', VoiceLevel::Sonorant);

		let mut word = Word::from_slice(&[m, a, m, a], NOUN);
		let rule = Rule2;
		rule.apply(&mut word);

		assert_eq!(word.to_string().as_str(), "meamea");
	}
}