use std::{
    ops::{Deref, DerefMut, Sub, SubAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
    iter::Map,
    vec::IntoIter as VecIntoIter,
    slice::Iter as SliceIter,
};
use crate::{
    Sound, 
    VoiceLevel,
    alphabet::{
        Alphabet, 
        alphabet::AlphabetUnit
    },
};

pub struct IntoIter {
    inner: Map<VecIntoIter<AlphabetUnit>, fn(AlphabetUnit) -> Sound>,
}

impl Iterator for IntoIter {
    type Item = Sound;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for Alphabet {
    type Item = Sound;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.storage.into_iter().map(|s| s.sound)
        }
    }
}

pub struct Iter<'a> {
    inner: Map<SliceIter<'a, AlphabetUnit>, fn(&'a AlphabetUnit) -> &'a Sound>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Sound;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a Alphabet {
    type Item = &'a Sound;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.storage.iter().map(|s| &s.sound)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Indexes {
    pub(crate) inner: Vec<usize>,
}

impl Deref for Indexes {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Indexes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl IntoIterator for Indexes {
    type Item = usize;
    type IntoIter = VecIntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Indexes {
    type Item = &'a usize;
    type IntoIter = SliceIter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Indexes {
    type Item = &'a mut usize;
    type IntoIter = std::slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VoiceLevelSet(pub(crate) u8);

impl VoiceLevelSet {
    pub const ALL: VoiceLevelSet = VoiceLevelSet(0b11_1111);
}

impl<Rhs> Sub<Rhs> for VoiceLevelSet
where
    Rhs: Into<VoiceLevelSet>
{
    type Output = VoiceLevelSet;

    fn sub(mut self, other: Rhs) -> Self::Output {
        self.0 &= !other.into().0;
        self
    }
}

impl<Rhs> SubAssign<Rhs> for VoiceLevelSet 
where
    Rhs: Into<VoiceLevelSet>
{
    fn sub_assign(&mut self, other: Rhs) {
        self.0 &= !(other.into().0) & 0b11_1111;
    }
}

impl<Rhs> BitAnd<Rhs> for VoiceLevelSet 
where
    Rhs: Into<VoiceLevelSet>
{
    type Output = VoiceLevelSet;

    fn bitand(mut self, other: Rhs) -> Self::Output {
        self.0 &= other.into().0;
        self
    }
}

impl<Rhs> BitAndAssign<Rhs> for VoiceLevelSet
where
    Rhs: Into<VoiceLevelSet>
{
    fn bitand_assign(&mut self, other: Rhs) {
        self.0 &= other.into().0;
    }
}

impl<Rhs> BitOr<Rhs> for VoiceLevelSet 
where
    Rhs: Into<VoiceLevelSet>
{
    type Output = VoiceLevelSet;

    fn bitor(mut self, other: Rhs) -> Self::Output {
        self.0 |= other.into().0;
        self
    }
}

impl<Rhs> BitOrAssign<Rhs> for VoiceLevelSet
where
    Rhs: Into<VoiceLevelSet>
{
    fn bitor_assign(&mut self, other: Rhs) {
        self.0 |= other.into().0;
    }
}

impl Not for VoiceLevelSet {
    type Output = VoiceLevelSet;

    fn not(self) -> Self::Output {
        VoiceLevelSet(!self.0 & 0b11_1111)
    }
}

impl From<VoiceLevel> for VoiceLevelSet {
    fn from(level: VoiceLevel) -> Self {
        Self(1 << (level as u8))
    } 
}

impl From<&[VoiceLevel]> for VoiceLevelSet {
    fn from(values: &[VoiceLevel]) -> Self {
        Self::from_iter(values.into_iter().copied())
    }
}

impl From<Vec<VoiceLevel>> for VoiceLevelSet {
    fn from(values: Vec<VoiceLevel>) -> Self {
        Self::from_iter(values.into_iter())
    }
}

impl<const N: usize> From<[VoiceLevel; N]> for VoiceLevelSet {
    fn from(values: [VoiceLevel; N]) -> Self {
        Self::from_iter(values.into_iter())
    }
}

impl FromIterator<VoiceLevel> for VoiceLevelSet {
    fn from_iter<T: IntoIterator<Item = VoiceLevel>>(iter: T) -> Self {
        let mut byte = 0u8;
        for value in iter.into_iter() {
            byte |= 1 << (value as u8);
        }
        Self(byte)
    }
}
