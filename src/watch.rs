use std::{
    mem::replace,
    ops::{Deref, DerefMut},
};

use crate::{
    lit::Lit,
    solver::{ClauseRef, Solver},
};
#[derive(Debug, Default, Clone)]
pub(crate) struct Watchlists {
    lists: Vec<Watchlist>,
}
#[derive(Debug, Default, Clone)]
pub(crate) struct Watchlist {
    list: Vec<Watch>,
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct Watch {
    pub(crate) cref: ClauseRef,
    pub(crate) blocking: Lit,
}

impl Watchlists {
    #[inline]
    pub(crate) fn resize(&mut self, var_count: usize) {
        self.lists.resize(var_count * 2, Watchlist::default());
    }

    #[inline]
    pub(crate) fn add_watch(&mut self, lit: Lit, watch: Watch) {
        self.lists[lit.code()].list.push(watch);
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

    #[inline]
    pub(crate) fn pop_watch_list(&mut self, lit: Lit) -> Watchlist {
        let watch_list = &mut self.lists[lit.code()];
        replace(watch_list, Watchlist::default())
    }

    #[inline]
    pub(crate) fn set_watch_list(&mut self, lit: Lit, watch_list: Watchlist) {
        self.lists[lit.code()] = watch_list;
    }
}
impl Watch {
    pub(crate) fn new(clause_ref: ClauseRef, blocking: Lit) -> Self {
        Watch {
            cref: clause_ref,
            blocking,
        }
    }
}
impl Deref for Watchlist {
    type Target = Vec<Watch>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
impl DerefMut for Watchlist {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
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
            let clause = &mut self.clause_db.long_clauses[index];
            self.watch_lists
                .watch_clause(ClauseRef::Long(index), clause[0], clause[1]);
        }
    }
}
