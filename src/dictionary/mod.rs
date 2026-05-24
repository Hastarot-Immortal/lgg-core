pub mod dictionary;
pub mod iter;

pub use dictionary::Dictionary;
pub use iter::{Words, WordsMut, DictIter, DictIterMut};

#[cfg(test)]
mod dictionary_test {
    use super::*;
    use crate::{ 
        Sound, 
        Word,
        PartOfSpeech
    };
    use crate::collections::*;

    fn create_simple_alphabet() -> (Sound, Sound, Sound, Sound) {
        (
            Sound::voiceless('s'),
            Sound::vowel('a'),
            Sound::sonorant('m'),
            Sound::voice('v')
        )
    }

    fn check_if_is_empty<I, M: Map<I, Word>>(map: &M) -> bool {
        map.is_empty()
    }

    fn fill_map<M: MapMut<u32, Word>>(map: &mut M) {
        let (s, a, m, v) = create_simple_alphabet();

        map.insert(1, Word::from_slice(&[m, a, s, s], PartOfSpeech::NOUN));
        map.insert(2, Word::from_slice(&[s, a, m], PartOfSpeech::NOUN));
        map.insert(3, Word::from_slice(&[s, a, m, s, a], PartOfSpeech::NOUN));
        map.insert(4, Word::from_slice(&[v, a, s, a], PartOfSpeech::NOUN));
    }

    #[test] 
    fn map() {
        let dict: Dictionary<String> = Dictionary::new();
        let is_empty = check_if_is_empty(&dict);
        assert_eq!(is_empty, true);
    }

    #[test]
    fn map_mut() {
        let (s, a, m, _) = create_simple_alphabet();

        let mut dict: Dictionary<u32> = Dictionary::new();
        fill_map(&mut dict);
        assert_eq!(dict.get(&2), Some(&Word::from_slice(&[s, a, m], PartOfSpeech::NOUN)));
    }

    #[test]
    fn from() {
        let (s, a, m, v) = create_simple_alphabet();

        let arr = [
            (-1i8, Word::from_slice(&[m, a, v], PartOfSpeech::NUM)),
            (0i8, Word::from_slice(&[s, a, v], PartOfSpeech::VERB)),
            (1i8, Word::from_slice(&[s, a, m], PartOfSpeech::CONJ)),
        ];

        let dict = Dictionary::from_array(arr);
        assert_eq!(dict.get(&0).map(|word: &Word| word.to_string()), Some("sav".to_string()))
    }

    #[test]
    fn from_iter() {
        let (s, a, m, v) = create_simple_alphabet();

        let vec = vec![
            (-1i8, Word::from_slice(&[m, a, v], PartOfSpeech::NUM)),
            (0i8, Word::from_slice(&[s, a, v], PartOfSpeech::VERB)),
            (1i8, Word::from_slice(&[s, a, m], PartOfSpeech::CONJ)),
        ];

        let dict: Dictionary<i8> = Dictionary::from_iter(vec);
        assert_eq!(dict.get(&-1).map(|word: &Word| word.to_string()), Some("mav".to_string()))
    }

    #[test]
    fn remove() {
        let mut dict = Dictionary::new();
        fill_map(&mut dict);
        let word = dict.remove(&1);
        assert_eq!(word.map(|w| w.to_string()), Some("mass".to_string()));
        assert_eq!(dict.get(&1), None);
    }

    #[test]
    fn clear() {
        let mut dict = Dictionary::new();
        fill_map(&mut dict);
        assert_eq!(dict.is_empty(), false);
        dict.clear();
        assert_eq!(dict.is_empty(), true);
    }

    #[test]
    fn words() {
        let mut dict = Dictionary::new();
        fill_map(&mut dict);
        let mut words = dict.words();
        assert!(words.next().is_some());
    }

    #[test]
    fn words_mut() {
        let (s, a, m, v) = create_simple_alphabet();
        let mut dict = Dictionary::new();
        dict.insert(1, Word::from_slice(&[m, a, s, s], PartOfSpeech::NOUN));
        for w in dict.words_mut() {
            w.push(v);
        }
        let mut iter = dict.iter();
        assert_eq!(iter.next(), Some((&1, &Word::from_slice(&[m, a, s, s, v], PartOfSpeech::NOUN))));
    }
}