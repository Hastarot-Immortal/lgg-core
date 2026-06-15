use crate::Word;

/// Defines a rule-based transformer that modifies a [`Word`] structure in-place.
///
/// Implementors can choose to mutate words dynamically via a method (`apply`) 
/// or using zero-overhead static definitions (`apply_static`) when state is not required.
pub trait Rule {
	/// Applies the mutation rule in-place to a specific mutable reference of a [`Word`].
	fn apply(&self, word: &mut Word);

	/// Applies a static, stateless variant of the mutation rule to a given [`Word`].
	fn apply_static(word: &mut Word) where Self: Sized;

	/// Iterates through a dynamically dispatched sequence of mutable words, applying this rule's instance method to each.
	fn apply_iter<'a>(&self, words: &mut dyn Iterator<Item = &'a mut Word>) {
		words.for_each(|word| self.apply(word));
	}
	
	/// Iterates through a statically dispatched sequence of mutable words, applying this rule's static method to each.
	fn apply_iter_static<'a, I>(words: &mut I) 
	where 
		I: Iterator<Item = &'a mut Word>,
		Self: Sized,
	{
		words.for_each(Self::apply_static);
	}
}

/// A declarative macro designed to generate [`Rule`] struct definitions smoothly.
///
/// This macro lets you construct stateless rules from closures, pass distinct logic pathways 
/// for dynamic and static contexts, or build rules containing stateful properties.
///
/// # Examples
///
/// ## 1. A Simple Stateless Rule
/// ```
/// use lgg_core::{rule, Rule, Word};
/// 
/// rule!(SwapRule, move |word: &mut Word| word.swap(0, 1));
/// ```
///
/// ## 2. A State-bearing Configurable Rule
/// ```
/// use lgg_core::{rule, Rule, Word};
///
/// rule!(
///     SoundRightRotation { shift_amount: usize },
///     move |word: &mut Word| { word.rotate_right(1); },
///     move |this: &SoundRightRotation, word: &mut Word| {
///         word.rotate_right(this.shift_amount);
///     }
/// );
/// ```
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
	use crate::{ Sound, PartOfSpeech::Noun, Dictionary };

	rule!(Rule1, move |word: &mut Word| word.swap(0, 1));
	
	fn swap_a_with_ea(word: &mut Word) {
		for sound in word.iter_mut() {
			if *sound == 'a' {
				*sound = Sound::try_vowel(['e', 'a']).unwrap();
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
		let a = Sound::vowel('a');
		let e = Sound::vowel('e');

		let mut word = Word::from_slice(&[a, e], Noun);
		let rule = Rule1;
		rule.apply(&mut word);
		assert_eq!(word, Word::from_slice(&[e, a], Noun));
	}

	#[test]
	fn rule2() {
		let a = Sound::vowel('a');
		let m = Sound::sonorant('m');

		let mut word = Word::from_slice(&[m, a, m, a], Noun);
		Rule2::apply_static(&mut word);

		assert_eq!(word.to_string().as_str(), "meamea");
	}

	#[test]
	fn rule3() {
		let a = Sound::vowel('a');
		let s = Sound::voiceless('s');
		let m = Sound::sonorant('m');

		let mut word1 = Word::from_slice(&[m, a, s, a], Noun);
		let mut word2 = word1.clone();

		let rule = Rule3 { param: 2 };
		rule.apply(&mut word1);
		Rule3::apply_static(&mut word2);

		assert_eq!(word1.to_string().as_str(), "sama");
		assert_eq!(word2.to_string().as_str(), "amsa");
	}

	#[test]
	fn rule_iter() {
		let a = Sound::vowel('a');
		let e = Sound::vowel('e');
		let m = Sound::sonorant('m');

		let mut words = vec![
			Word::from_slice(&[e, m, m, a], Noun),
			Word::from_slice(&[m, e, m, a], Noun),
			Word::from_slice(&[a, m, m, a], Noun),
		];
		let rule = Rule2;
		rule.apply_iter(&mut words.iter_mut());
		assert_eq!(words[0].to_string().as_str(), "emmea");
		assert_eq!(words[1].to_string().as_str(), "memea");
		assert_eq!(words[2].to_string().as_str(), "eammea");
	}

	#[test]
	fn rule_dict() {
		let a = Sound::vowel('a');
		let e = Sound::vowel('e');
		let m = Sound::sonorant('m');

		let mut dict = Dictionary::from_vec(vec![
			(1, Word::from_slice(&[a, m, m, a], Noun)),
			(2, Word::from_slice(&[e, m, a], Noun)),
			(3, Word::from_slice(&[a, m, a], Noun)),
		]);
		
		Rule2::apply_iter_static(&mut dict.iter_mut().map(|(_, w)| w));

		assert_eq!(dict.get(&1).map(|w| w.to_string()), Some("eammea".to_string()));
		assert_eq!(dict.get(&2).map(|w| w.to_string()), Some("emea".to_string()));
		assert_eq!(dict.get(&3).map(|w| w.to_string()), Some("eamea".to_string()));
	}

	#[test]
	fn rule_static() {
		let a = Sound::vowel('a');
		let e = Sound::vowel('e');
		let m = Sound::sonorant('m');

		let mut word1 = Word::from_slice(&[e, m, m, a], Noun);
		let mut word2 = word1.clone();

		let rule = Rule2;
		rule.apply(&mut word1);
		Rule2::apply_static(&mut word2);

		assert_eq!(word1, word2);
	}
}
