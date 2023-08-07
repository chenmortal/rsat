use std::{collections::HashSet, ops::Range, option::IterMut, path::Iter};

use super::lit::Lit;
use anyhow::{bail, Result};
#[derive(Clone, Default,Debug)]
pub struct CnfFormula {
    lit_count: usize,
    literals: Vec<Lit>,
    clause_ranges: Vec<Range<usize>>,
}
impl CnfFormula {
    pub fn new() -> Self {
        CnfFormula::default()
    }
    pub fn iter(&self) -> impl Iterator<Item = &[Lit]> {
        let lits = &self.literals;
        self.clause_ranges.iter().map(|range| &lits[range.clone()])
    }
    pub fn add_clause(&mut self, clause: &[Lit]) {
        if let Ok(clause) = self.check_clause(clause) {
            let start = self.literals.len();
            self.literals.extend_from_slice(&clause);
            let end = self.literals.len();
            self.clause_ranges.push(start..end);
        };
    }
    #[inline]
    fn check_clause(&mut self, clause: &[Lit]) -> Result<Vec<Lit>> {
        let mut check = HashSet::<Lit>::new();
        let mut res = Vec::new();
        for ele in clause {
            if check.contains(&!*ele) {
                bail!("this clause has a and !a , no sense");
            }
            if !check.contains(ele) {
                check.insert(*ele);
                self.lit_count = ele.index().max(self.lit_count);
                res.push(*ele);
            }
        }
        Ok(res)
    }
    #[inline]
    pub(super) fn get_lit_count(&self) -> usize {
        self.lit_count+1
    }
}
#[cfg(test)]
mod tests {
    // use crate::sat::lit::Lit;
    

    use crate::lit::Lit;

    use super::CnfFormula;

    #[test]
    fn test_add_clause() {
        let mut formula = CnfFormula::default();
        let a = Lit::from_dimacs(1);
        let b = Lit::from_dimacs(2);
        let c = Lit::from_dimacs(3);
        formula.add_clause(&[a, a, b]);
        formula.add_clause(&[b, c]);
        formula.add_clause(&[!a, b]);
        formula.add_clause(&[a, !a]);
        let mut formula_iter = formula.iter();
        assert_eq!(formula_iter.next(), Some([a, b].as_ref()));
        assert_eq!(formula_iter.next(), Some([b, c].as_ref()));
        assert_eq!(formula_iter.next(), Some([!a, b].as_ref()));
        assert_eq!(formula_iter.next(), None);
    }
}
