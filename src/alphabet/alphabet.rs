use crate::{
    alphabet::{
        iter::Indexes,
        index::{AlphabetIndex, AlphabetIndexOwned},
    },
    Sound,
    sound::{AsSound, TryAsSound},
};
use std::ops::Index;

pub type AlphabetKey = [u8; 6];

#[derive(Debug, Clone)]
pub(super) struct AlphabetUnit {
    key: AlphabetKey,
    pub(super) sound: Sound,
    left: Option<usize>,
    right: Option<usize>,
}

pub struct Alphabet {
    pub(super) storage: Vec<AlphabetUnit>,
    root: Option<usize>,
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
                .map(|node| AlphabetUnit {
                    key: node.key,
                    sound: node.value,
                    left: node.left,
                    right: node.right,
                })
                .collect(),
            root,
        }
    }
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

impl Alphabet {
    pub fn get<I: AlphabetIndex>(&self, idx: I) -> Option<&I::Output> {
        idx.get(self)
    }

    pub fn get_owned<I: AlphabetIndexOwned>(&self, idx: I) -> Option<I::Owned> {
        idx.get_owned(self)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn indexes(&self) -> Indexes {
        Indexes {
            inner: (0..self.len()).collect(),
        }
    }

    pub fn indexes_by<F: Fn(&Sound) -> bool>(&self, f: F) -> Indexes {
        Indexes {
            inner: self
                .storage
                .iter()
                .enumerate()
                .filter_map(|(i, u)| if f(&u.sound) { Some(i) } else { None })
                .collect(),
        }
    }
}

impl<I> Index<I> for Alphabet 
where
    I: AlphabetIndex
{
    type Output = I::Output;
    fn index(&self, idx: I) -> &Self::Output {
        idx.index(self)
    }
}

pub(super) fn search<K: TryAsSound>(alphabet: &Alphabet, key: K) -> Option<&Sound> {
    match key.try_as_sound() {
        Ok(key) => avl::search(&alphabet.storage, alphabet.root, key),
        Err(_) => None,
    }
}

mod avl {
    use super::*;
    use std::cmp::Ordering;

    #[derive(Debug)]
    pub(super) struct AVLNode {
        pub(crate) key: AlphabetKey,
        pub(crate) value: Sound,
        pub(crate) left: Option<usize>,
        pub(crate) right: Option<usize>,
        height: i8,
    }

    impl From<Sound> for AVLNode {
        fn from(sound: Sound) -> Self {
            Self {
                key: sound.as_sound(),
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

    pub(super) fn search(
        storage: &Vec<AlphabetUnit>,
        idx: Option<usize>,
        key: AlphabetKey,
    ) -> Option<&Sound> {
        if let Some(idx) = idx {
            match Ord::cmp(&storage[idx].key, &key) {
                Ordering::Equal => Some(&storage[idx].sound),
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
            match Ord::cmp(&storage[idx].key, &value.as_sound()) {
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
