use crate::{
    lit::Lit,
    solver::{ClauseRef, Solver},
};
#[derive(Debug, Default, Clone)]
pub(crate) struct Watchlists(Vec<Vec<Watch>>);
#[derive(Debug, Clone, Copy)]
pub(crate) struct Watch {
    cref: ClauseRef,
    blocking: Lit,
}

impl Watchlists {
    #[inline]
    pub(crate) fn resize(&mut self, var_count: usize) {
        self.0.resize(var_count * 2, vec![]);
    }
    #[inline]
    pub(crate) fn add_watch(&mut self, lit: Lit, watch: Watch) {
        self.0[lit.code()].push(watch);
    }
    #[inline]
    pub(crate) fn watch_clause(&mut self, clause_ref: ClauseRef, first: Lit, second: Lit) {
        self.add_watch(
            !first,
            Watch {
                cref: clause_ref,
                blocking: second,
            },
        );
        self.add_watch(
            !second,
            Watch {
                cref: clause_ref,
                blocking: first,
            },
        );
    }
}
impl Solver {
    pub(crate) fn generate_watch(&mut self) {
        for index in 0..self.clause_db.binary_clauses.len() {
            let clause = self.clause_db.binary_clauses[index].as_ref();
            self.watch_lists
                .watch_clause(ClauseRef::Binary(index), clause[0], clause[1]);
        }
        for index in 0..self.clause_db.long_clauses.len() {
            let clause = self.clause_db.long_clauses[index].as_ref();
            self.watch_lists
                .watch_clause(ClauseRef::Long(index), clause[0], clause[1]);
        }
    }
}
