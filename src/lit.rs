use std::{fmt, ops};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lit(usize);
impl Lit {
    #[inline]
    pub fn from_dimacs(number: usize, polarity: bool) -> Self {
        debug_assert!(number >= 1);
        Self::from_index(number - 1, polarity)
    }
    #[inline]
    pub fn from_index(number: usize, polarity: bool) -> Self {
        debug_assert!(number <= (usize::MAX >> 2));
        Self(number << 1 | (!polarity as usize))
    }
    #[inline]
    pub fn is_negative(self) -> bool {
        (self.0 & 1) != 0
    }
    #[inline]
    pub fn is_positive(self) -> bool {
        !self.is_negative()
    }
    #[inline]
    pub fn index(self) -> usize {
        self.0 >> 1
    }
    #[inline]
    pub(super) fn code(self) -> usize {
        self.0
    }
    #[inline]
    pub fn to_dimacs(self) -> usize {
        self.index() + 1
    }
    #[inline]
    pub fn assign_bool(&self, assignment: &Vec<Option<bool>>) -> Option<bool> {
        match assignment[self.index()] {
            Some(assign) => Some(assign == self.is_positive()),
            None => None,
        }
    }
}
impl ops::Not for Lit {
    type Output = Lit;

    fn not(self) -> Self::Output {
        Self(self.0 ^ 1)
    }
}
impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative() {
            f.write_fmt(format_args!("¬{}", self.to_dimacs()))
        } else {
            f.write_fmt(format_args!("{}", self.to_dimacs()))
        }
    }
}
