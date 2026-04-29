use crate::Word;

pub trait Rule {
	fn apply(&self, word: &mut Word);

	fn apply_static(word: &mut Word) where Self: Sized;

	fn apply_iter<'a>(&self, words: &mut dyn Iterator<Item = &'a mut Word>) {
		for word in words {
			self.apply(word);
		}
	}
	
	fn apply_iter_static<'a, I>(words: &mut I) 
	where 
		I: Iterator<Item = &'a mut Word>,
		Self: Sized,
	{
		for word in words {
			Self::apply_static(word);
		}
	}
}

#[macro_export]
macro_rules! rule {
	($name: ident, $func: expr) => {
		pub struct $name;

		impl Rule for $name {
			fn apply_static(word: &mut Word) {
				($func)(word)
			}

			fn apply(&self, word: &mut Word) {
				($func)(word)
			}
		}
	};
	($(#[$meta:meta])* $name: ident, $func: expr) => {
		$(#[$meta])*
		pub struct $name;

		impl Rule for $name {
			fn apply_static(word: &mut Word) {
				($func)(word)
			}

			fn apply(&self, word: &mut Word) {
				($func)(word)
			}
		}
	};
	(
		$name: ident, 
		$static_func: expr, 
		$func: expr
	) => {
		pub struct $name;

		impl Rule for $name {
			fn apply_static(word: &mut Word) {
				($static_func)(word)
			}

			fn apply(&self, word: &mut Word) {
				($func)(self, word)
			}
		}
	};
	(
		$(#[$meta:meta])* 
		$name: ident, 
		$static_func: expr, 
		$func: expr
	) => {
		$(#[$meta])*
		pub struct $name;

		impl Rule for $name {
			fn apply_static(word: &mut Word) {
				($static_func)(word)
			}

			fn apply(&self, word: &mut Word) {
				($func)(self, word)
			}
		}
	};
	(
		$name:ident { $($field:ident : $type:ty),* $(,)? }, 
		$static_func: expr, 
		$func: expr
	) => {
		pub struct $name {
            $( pub $field: $type, )*
        }

		impl Rule for $name {
			fn apply_static(word: &mut Word) {
				($static_func)(word)
			}

			fn apply(&self, word: &mut Word) {
				($func)(self, word)
			}
		}
	};
	(
		$(#[$meta:meta])* 
		$name:ident { $($field:ident : $type:ty),* $(,)? }, 
		$static_func: expr, 
		$func: expr
	) => {
		$(#[$meta])*
		pub struct $name {
            $( pub $field: $type, )*
        }

		impl Rule for $name {
			fn apply_static(word: &mut Word) {
				($static_func)(word)
			}

			fn apply(&self, word: &mut Word) {
				($func)(self, word)
			}
		}
	};
}

#[cfg(test)]
mod rule_test {
	use super::*;
	use crate::{ Sound, PartOfSpeech::NOUN, VoiceLevel, Dictionary };

	rule!(Rule1, move |word: &mut Word| word.swap(0, 1));
	
	fn swap_a_with_ea(word: &mut Word) {
		for sound in word.iter_mut() {
			if *sound == 'a' {
				*sound = Sound::diphthong(['e', 'a']);
			}
		}
	}

	rule!(#[derive(Debug)]Rule2, swap_a_with_ea);

	rule!(
		#[derive(Debug)]
		Rule3 { param: usize }, 
		move |word: &mut Word| word.swap(0, 1),
		move |rule: &Rule3, word: &mut Word| word.swap(0, rule.param)
	);

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
		Rule2::apply_static(&mut word);

		assert_eq!(word.to_string().as_str(), "meamea");
	}

	#[test]
	fn rule3() {
		let a = Sound::monophthong('a');
		let s = Sound::new('s', VoiceLevel::Voiceless);
		let m = Sound::new('m', VoiceLevel::Sonorant);

		let mut word1 = Word::from_slice(&[m, a, s, a], NOUN);
		let mut word2 = word1.clone();

		let rule = Rule3 { param: 2 };
		rule.apply(&mut word1);
		Rule3::apply_static(&mut word2);

		assert_eq!(word1.to_string().as_str(), "sama");
		assert_eq!(word2.to_string().as_str(), "amsa");
	}

	#[test]
	fn rule_iter() {
		let a = Sound::monophthong('a');
		let e = Sound::monophthong('e');
		let m = Sound::new('m', VoiceLevel::Sonorant);

		let mut words = vec![
			Word::from_slice(&[e, m, m, a], NOUN),
			Word::from_slice(&[m, e, m, a], NOUN),
			Word::from_slice(&[a, m, m, a], NOUN),
		];
		let rule = Rule2;
		rule.apply_iter(&mut words.iter_mut());
		assert_eq!(words[0].to_string().as_str(), "emmea");
		assert_eq!(words[1].to_string().as_str(), "memea");
		assert_eq!(words[2].to_string().as_str(), "eammea");
	}

	#[test]
	fn rule_dict() {
		let a = Sound::monophthong('a');
		let e = Sound::monophthong('e');
		let m = Sound::new('m', VoiceLevel::Sonorant);

		let mut dict = Dictionary::from_vec(vec![
			(1, Word::from_slice(&[a, m, m, a], NOUN)),
			(2, Word::from_slice(&[e, m, a], NOUN)),
			(3, Word::from_slice(&[a, m, a], NOUN)),
		]);
		
		Rule2::apply_iter_static(&mut dict.iter_mut().map(|(_, w)| w));

		assert_eq!(dict.get(&1).map(|w| w.to_string()), Some("eammea".to_string()));
		assert_eq!(dict.get(&2).map(|w| w.to_string()), Some("emea".to_string()));
		assert_eq!(dict.get(&3).map(|w| w.to_string()), Some("eamea".to_string()));
	}

	#[test]
	fn rule_static() {
		let a = Sound::monophthong('a');
		let e = Sound::monophthong('e');
		let m = Sound::new('m', VoiceLevel::Sonorant);

		let mut word1 = Word::from_slice(&[e, m, m, a], NOUN);
		let mut word2 = word1.clone();

		let rule = Rule2;
		rule.apply(&mut word1);
		Rule2::apply_static(&mut word2);

		assert_eq!(word1, word2);
	}
}