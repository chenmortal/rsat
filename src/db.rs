use crate::{assign::Assignment, cnf::CnfFormula, lit::Lit};
use std::{cmp::Ordering, ops::Deref, os::fd::IntoRawFd, slice::Iter};
#[derive(Debug, Clone, Copy)]
pub(crate) struct AssignClause(pub(crate) Lit);
#[derive(Debug)]
pub(crate) struct BinaryClause {
    pub(crate) clause: [Lit; 2],
    pub(crate) state: Option<bool>,
}
#[derive(Debug,)]
pub(crate) struct LongClause {
    pub(crate) clause: Vec<Lit>,
    pub(crate) state: Option<bool>,
}
pub(crate) trait Clause {
    fn get_clause_iter(&self) -> Iter<'_, Lit>;
    fn check_satisfied(&mut self, assignment: &Assignment) -> Option<bool> {
        let mut exist_none = false;
        for lit in self.get_clause_iter() {
            match assignment.assign_bool(lit) {
                Some(b) => {
                    if b {
                        self.update_state(Some(true));
                        return self.get_state();
                    }
                }
                None => {
                    exist_none = true;
                }
            };
        }
        self.update_state(if exist_none { None } else { Some(false) });
        self.get_state()
    }

    fn update_state(&mut self, state: Option<bool>);
    fn get_state(&mut self) -> Option<bool>;
}
impl Clause for BinaryClause {
    #[inline]
    fn get_clause_iter(&self) -> Iter<'_, Lit> {
        self.clause.iter()
    }
    #[inline]
    fn update_state(&mut self, state: Option<bool>) {
        self.state = state;
    }
    #[inline]
    fn get_state(&mut self) -> Option<bool> {
        self.state
    }
}
impl Clause for LongClause {
    #[inline]
    fn get_clause_iter(&self) -> Iter<'_, Lit> {
        self.clause.iter()
    }
    #[inline]
    fn update_state(&mut self, state: Option<bool>) {
        self.state = state;
    }
    #[inline]
    fn get_state(&mut self) -> Option<bool> {
        self.state
    }
}
impl AsMut<BinaryClause> for BinaryClause {
    fn as_mut(&mut self) -> &mut BinaryClause {
        self
    }
}
impl AsMut<LongClause> for LongClause {
    fn as_mut(&mut self) -> &mut LongClause {
        self
    }
}
impl Deref for BinaryClause {
    type Target = [Lit; 2];

    fn deref(&self) -> &Self::Target {
        &self.clause
    }
}
impl Deref for LongClause {
    type Target = Vec<Lit>;

    fn deref(&self) -> &Self::Target {
        &self.clause
    }
}
impl AsRef<Vec<Lit>> for LongClause {
    fn as_ref(&self) -> &Vec<Lit> {
        &self.clause
    }
}
#[derive(Debug, Default)]
pub(crate) struct ClauseDb {
    pub(crate) assign_clauses: Vec<AssignClause>,
    pub(crate) binary_clauses: Vec<BinaryClause>,
    pub(crate) long_clauses: Vec<LongClause>,
}
impl ClauseDb {
    pub(crate) fn add_formula(&mut self, formula: &CnfFormula) {
        for clause in formula.iter() {
            match clause {
                [] => {}
                [lit] => self.assign_clauses.push(AssignClause(*lit)),
                [m, n] => self.binary_clauses.push(BinaryClause {
                    clause: [*m, *n],
                    state: None,
                }),

                _ => self.long_clauses.push(LongClause {
                    clause: clause.to_vec(),
                    state: None,
                }),
            }
        }
    }
}
