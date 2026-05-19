use crate::{ Sound, sound::VowelSymbol };
use std::ops::Index;

#[derive(Debug, Clone)]
struct AlphabetUnit {
    key: [char; 3],
    sound: Sound,
    left: Option<usize>,
    right: Option<usize>,
}

impl From<avl::AVLNode> for AlphabetUnit {
    fn from(node: avl::AVLNode) -> Self {
        Self {
            key: node.key,
            sound: node.value,
            left: node.left,
            right: node.right,
        }
    }
}

pub struct Alphabet {
    storage: Vec<AlphabetUnit>,
    root: Option<usize>,
}

impl<const N: usize> From<[Sound; N]> for Alphabet {
    fn from(sounds: [Sound; N]) -> Self {
        Self::from_iter(sounds)
    }
}

impl From<Vec<Sound>> for Alphabet {
    fn from(sounds: Vec<Sound>) -> Self {
        Self::from_iter(sounds)
    }
}

impl FromIterator<Sound> for Alphabet {
    fn from_iter<T: IntoIterator<Item = Sound>>(sounds: T) -> Self {
        let mut storage = Vec::new();
        let mut root = None;
        for sound in sounds {
            root = Some(avl::insert(sound, &mut storage, root));
        }
        Self {
            storage: storage
                .into_iter()
                .map(|node| AlphabetUnit::from(node))
                .collect(),
            root,
        }
    }
}

impl IntoIterator for Alphabet {
    type Item = Sound;

    type IntoIter = std::vec::IntoIter<Sound>;

    fn into_iter(self) -> Self::IntoIter {
        let sounds: Vec<Sound> = self.as_vec();
        sounds.into_iter()
    }
}

impl Alphabet {
    pub fn get<K: AlphabetKey>(&self, key: &K) -> Option<Sound> {
        avl::search(&self.storage, self.root, &key.as_key())
    }

    pub fn as_vec(&self) -> Vec<Sound> {
        self.storage.clone().into_iter().map(|s| s.sound).collect()
    }
}

impl Index<usize> for Alphabet {
    type Output = Sound;
    fn index(&self, index: usize) -> &Self::Output {
        &self.storage[index].sound
    }
}

pub trait AlphabetKey {
    fn as_key(&self) -> [char; 3];
}

impl AlphabetKey for Sound {
    fn as_key(&self) -> [char; 3] {
        match *self {
            Sound::Consonant(ch, _) => ch.as_key(),
            Sound::Vowel(vowel_symbol) => match vowel_symbol {
                VowelSymbol::Monophthong(ch) => ch.as_key(),
                VowelSymbol::Diphthong(ch) => ch.as_key(),
                VowelSymbol::Triphthong(ch) => ch.as_key(),
            },
        }
    }
}

impl AlphabetKey for char {
    fn as_key(&self) -> [char; 3] {
        [*self, '\0', '\0']
    }
}

impl AlphabetKey for [char; 2] {
    fn as_key(&self) -> [char; 3] {
        [self[0], self[1], '\0']
    }
}

impl AlphabetKey for [char; 3] {
    fn as_key(&self) -> [char; 3] {
        *self
    }
}

impl AlphabetKey for &str {
    fn as_key(&self) -> [char; 3] {
        let mut res = ['\0'; 3];
        let iter = self.chars().take(3).enumerate();
        for (i, c) in iter {
            res[i] = c;
        }
        res
    }
}

mod avl {
    use crate::Sound;
    use super::{ AlphabetUnit, AlphabetKey };
    use std::cmp::Ordering;

    #[derive(Debug)]
    pub struct AVLNode {
        pub key: [char; 3],
        pub value: Sound,
        pub left: Option<usize>,
        pub right: Option<usize>,
        height: i8,
    }

    impl From<Sound> for AVLNode {
        fn from(sound: Sound) -> Self {
            Self {
                key: sound.as_key(),
                value: sound,
                left: None,
                right: None,
                height: 1,
            }
        }
    }

    impl From<&Sound> for AVLNode {
        fn from(value: &Sound) -> Self {
            Self::from(*value)
        }
    }

    pub(super) fn search(storage: &Vec<AlphabetUnit>, idx: Option<usize>, key: &[char; 3]) -> Option<Sound> {
        if let Some(idx) = idx {
            match Ord::cmp(&storage[idx].key, key) {
                Ordering::Equal => Some(storage[idx].sound),
                Ordering::Less => search(storage, storage[idx].right, key),
                Ordering::Greater => search(storage, storage[idx].left, key),
            }
        } else {
            None
        }
    }

    fn height(storage: &Vec<AVLNode>, idx: Option<usize>) -> i8 {
        idx.map(|i| storage[i].height).unwrap_or_default()
    }

    fn update_height(storage: &mut Vec<AVLNode>, idx: usize) {
        storage[idx].height = 1 + Ord::max(
            height(storage, storage[idx].left),
            height(storage, storage[idx].right),
        );
    }

    fn balance_factor(storage: &Vec<AVLNode>, idx: usize) -> i8 {
        height(storage, storage[idx].left).saturating_sub(height(storage, storage[idx].right))
    }

    pub(super) fn insert(value: Sound, storage: &mut Vec<AVLNode>, idx: Option<usize>) -> usize {
        if let Some(idx) = idx {
            match Ord::cmp(&storage[idx].key, &value.as_key()) {
                Ordering::Equal => return idx,
                Ordering::Less => {
                    let right = storage[idx].right;
                    let new_right = insert(value, storage, right);
                    storage[idx].right = Some(new_right);
                }
                Ordering::Greater => {
                    let left = storage[idx].left;
                    let new_left = insert(value, storage, left);
                    storage[idx].left = Some(new_left);
                }
            }
            rebalance(storage, idx)
        } else {
            let new_idx = storage.len();
            storage.push(AVLNode::from(value));
            new_idx
        }
    }

    fn rebalance(storage: &mut Vec<AVLNode>, idx: usize) -> usize {
        update_height(storage, idx);
        let balance = balance_factor(storage, idx);
        if balance > 1 {
            let left_idx = storage[idx].left;
            if let Some(left_idx) = left_idx {
                if balance_factor(storage, left_idx) >= 0 {
                    right_rotate(storage, idx)
                } else {
                    let new_left = left_rotate(storage, left_idx);
                    storage[idx].left = Some(new_left);
                    right_rotate(storage, idx)
                }
            } else {
                idx
            }
        } else if balance < -1 {
            let right_idx = storage[idx].right;
            if let Some(right_idx) = right_idx {
                if balance_factor(storage, right_idx) <= 0 {
                    left_rotate(storage, idx)
                } else {
                    let new_right = right_rotate(storage, right_idx);
                    storage[idx].right = Some(new_right);
                    left_rotate(storage, idx)
                }
            } else {
                idx
            }
        } else {
            idx
        }
    }

    fn right_rotate(storage: &mut Vec<AVLNode>, idx: usize) -> usize {
        let left = storage[idx].left;
        if let Some(left) = left {
            let left_right = storage[left].right;
            storage[left].right = Some(idx);
            storage[idx].left = left_right;
            update_height(storage, idx);
            update_height(storage, left);
            left
        } else {
            idx
        }
    }

    fn left_rotate(storage: &mut Vec<AVLNode>, idx: usize) -> usize {
        let right = storage[idx].right;
        if let Some(right) = right {
            let right_left = storage[right].left;
            storage[right].left = Some(idx);
            storage[idx].right = right_left;
            update_height(storage, idx);
            update_height(storage, right);
            right
        } else {
            idx
        }
    }
}
