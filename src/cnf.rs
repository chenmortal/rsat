use std::{collections::HashSet, ops::Range};

use super::lit::Lit;
use anyhow::{bail, Result};
#[derive(Clone, Default, Debug)]
pub struct CnfFormula {
    max_lit_index: usize,
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
                self.max_lit_index = ele.index().max(self.max_lit_index);
                res.push(*ele);
            }
        }
        Ok(res)
    }
    #[inline]
    pub(super) fn get_max_lit_index(&self) -> usize {
        self.max_lit_index
    }
}
