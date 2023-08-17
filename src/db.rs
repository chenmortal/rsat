use crate::{cnf::CnfFormula, lit::Lit, solver::ClauseRef};

#[derive(Debug, Default)]
pub(crate) struct ClauseDb {
    pub(crate) assign_clauses: Vec<Lit>,
    pub(crate) binary_clauses: Vec<[Lit; 2]>,
    pub(crate) long_clauses: Vec<Vec<Lit>>,
}
impl ClauseDb {
    #[inline]
    pub(crate) fn add_formula(&mut self, formula: &CnfFormula) {
        for clause in formula.iter() {
            self.add_clause(clause);
        }
    }
    #[inline]
    pub(crate) fn add_clause(&mut self, clause: &[Lit]) -> Option<ClauseRef> {
        match clause {
            [] => None,
            [lit] => {
                self.assign_clauses.push(*lit);
                None
            }
            [m, n] => {
                self.binary_clauses.push([*m, *n]);
                Some(ClauseRef::Binary(self.binary_clauses.len() - 1))
            }
            _ => {
                self.long_clauses.push(clause.to_vec());
                Some(ClauseRef::Long(self.long_clauses.len() - 1))
            }
        }
    }
}
