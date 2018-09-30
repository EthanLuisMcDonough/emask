#[macro_use]
mod macros;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

pub trait MaskValue:
    Clone
    + Send
    + Sync
    + fmt::Debug
    + Into<isize>
    + PartialEq
    + Not<Output = MaskManager<Self>>
    + BitOr<Output = MaskManager<Self>>
    + BitXor<Output = MaskManager<Self>>
    + BitAnd<Output = MaskManager<Self>>
{
    fn value(&self) -> isize;

    fn max_val() -> isize;

    fn owned_values() -> Vec<Self>;

    fn all() -> MaskManager<Self> {
        MaskManager::all()
    }

    fn empty() -> MaskManager<Self> {
        MaskManager::empty()
    }

    /// Converts the value into a manager
    fn into_manager(self) -> MaskManager<Self> {
        MaskManager::new_with(self.into())
    }
}

#[derive(Clone, PartialEq)]
pub struct MaskManager<T: MaskValue> {
    value: isize,
    placeholder: Option<T>,
}

impl<T: MaskValue> MaskManager<T> {
    pub fn empty() -> Self {
        Self::new_with(0)
    }

    pub fn all() -> Self {
        Self::new_with(T::max_val())
    }

    fn new_with(value: isize) -> Self {
        Self {
            value,
            placeholder: None,
        }
    }

    fn is_full(&self) -> bool {
        self.value() == T::max_val()
    }

    fn is_empty(&self) -> bool {
        self.value() == 0
    }

    pub fn value(&self) -> isize {
        self.value
    }

    pub fn clear(&mut self) {
        self.value = 0;
    }

    pub fn invert(&mut self) {
        self.value = !self.value;
    }

    pub fn set(&mut self, set: impl Into<Self>) {
        self.value |= isize::from(set.into());
    }

    pub fn toggle(&mut self, toggle: impl Into<Self>) {
        self.value ^= isize::from(toggle.into());
    }

    pub fn is_set(&self, check: impl Into<Self>) -> bool {
        let manager = check.into();
        (self.value & manager.value()) == manager.into()
    }

    pub fn remove(&mut self, to_rem: impl Into<Self>) {
        self.value &= !isize::from(to_rem.into());
    }
}

impl<T: MaskValue> PartialEq<T> for MaskManager<T> {
    fn eq(&self, v: &T) -> bool {
        self.value == v.value()
    }
}

impl<T: MaskValue> From<T> for MaskManager<T> {
    fn from(value: T) -> MaskManager<T> {
        value.into_manager()
    }
}

impl<T: MaskValue> fmt::Debug for MaskManager<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "MaskManager(0b{:0len$b})",
            self.value,
            len = T::max_val().count_ones() as usize
        )
    }
}

impl<T: MaskValue> From<MaskManager<T>> for isize {
    fn from(manager: MaskManager<T>) -> isize {
        manager.value as isize
    }
}

impl<T: MaskValue> Not for MaskManager<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        MaskManager::new_with(!self.value)
    }
}

impl<T: MaskValue, I: Into<Self>> BitOr<I> for MaskManager<T> {
    type Output = Self;

    fn bitor(mut self, other: I) -> Self::Output {
        self |= other;
        self
    }
}

impl<T: MaskValue, I: Into<Self>> BitXor<I> for MaskManager<T> {
    type Output = Self;

    fn bitxor(mut self, other: I) -> Self::Output {
        self ^= other;
        self
    }
}

impl<T: MaskValue, I: Into<Self>> BitAnd<I> for MaskManager<T> {
    type Output = Self;

    fn bitand(mut self, other: I) -> Self::Output {
        self &= other;
        self
    }
}

impl<T: MaskValue, I: Into<Self>> BitOrAssign<I> for MaskManager<T> {
    fn bitor_assign(&mut self, other: I) {
        self.set(other)
    }
}

impl<T: MaskValue, I: Into<Self>> BitXorAssign<I> for MaskManager<T> {
    fn bitxor_assign(&mut self, other: I) {
        self.toggle(other)
    }
}

impl<T: MaskValue, I: Into<Self>> BitAndAssign<I> for MaskManager<T> {
    fn bitand_assign(&mut self, other: I) {
        self.value &= isize::from(other.into());
    }
}

mod A {
    use super::{MaskManager, MaskValue};
    emask!(pub Thing { A, B, C });
}

emask!(pub Thing { A, B, C });

fn main() {
    let mut h = Thing::all();

    println!("{:?}", h.clone() & !Thing::C);
}
