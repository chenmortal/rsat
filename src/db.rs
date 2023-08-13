use crate::{cnf::CnfFormula, lit::Lit};

#[derive(Debug, Default)]
pub(crate) struct ClauseDb {
    pub(crate) assign_clauses: Vec<Lit>,
    pub(crate) binary_clauses: Vec<[Lit; 2]>,
    pub(crate) long_clauses: Vec<Vec<Lit>>,
}
impl ClauseDb {
    pub(crate) fn add_formula(&mut self, formula: &CnfFormula) {
        for clause in formula.iter() {
            match clause {
                [] => {}
                [lit] => self.assign_clauses.push(*lit),
                [m, n] => self.binary_clauses.push([*m, *n]),
                _ => self.long_clauses.push(clause.to_vec()),
            }
        }
    }
}
