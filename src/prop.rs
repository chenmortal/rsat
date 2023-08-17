use std::ops::Deref;

use crate::{
    cdcl::Conflict,
    graph::PropReason,
    lit::Lit,
    solver::{ClauseRef, Solver},
    watch::Watch,
};
use anyhow::Result;
#[derive(Debug, Default)]
pub(crate) struct PropQueue {
    trail: Vec<Lit>,
    pos: usize,
    level_with_trail_len: Vec<usize>,
}
impl PropQueue {
    pub(super) fn pop_queue(&mut self) -> Option<Lit> {
        if let Some(s) = self.trail.get(self.pos) {
            self.pos += 1;
            Some(*s)
        } else {
            None
        }
    }
    pub(super) fn push_back(&mut self, lit: &Lit) {
        self.trail.push(*lit);
    }
    pub(crate) fn new_decision_level(&mut self) {
        self.level_with_trail_len.push(self.trail.len());
    }
    pub(crate) fn current_level(&self) -> usize {
        self.level_with_trail_len.len()
    }
}
impl From<&[Lit]> for PropQueue {
    fn from(value: &[Lit]) -> Self {
        PropQueue {
            trail: value.to_vec(),
            pos: 0,
            level_with_trail_len: vec![],
        }
    }
}
impl Deref for PropQueue {
    type Target = Vec<Lit>;

    fn deref(&self) -> &Self::Target {
        &self.trail
    }
}
impl Solver {
    pub(crate) fn propagate(&mut self) -> Result<(), Conflict> {
        while let Some(lit) = self.prop_queue.pop_queue() {
            let mut watch_list = self.watch_lists.pop_watch_list(lit);
            let mut i = 0;
            'watch: loop {
                if i == watch_list.len() {
                    break;
                }
                let watch = watch_list.get_mut(i).unwrap();
                i += 1;
                match watch.cref {
                    ClauseRef::Binary(_) => match self.assignment.value(&watch.blocking) {
                        Some(false) => {
                            return Err(Conflict::Binary([!lit, watch.blocking]));
                        }
                        None => {
                            self.add_assign(&watch.blocking, PropReason::Binary([!lit]));
                        }
                        Some(true) => {}
                    },
                    ClauseRef::Long(index) => {
                        if self.assignment.is_true(&watch.blocking) {
                            continue;
                        }
                        let clause = &mut self.clause_db.long_clauses[index];

                        if clause[0] == !lit {
                            clause.swap(0, 1);
                        }
                        let new_watch = Watch::new(watch.cref, clause[0]);
                        if clause[0] != watch.blocking && self.assignment.is_true(&clause[0]) {
                            *watch = new_watch;
                            continue;
                        }
                        for lit_index in 2..clause.len() {
                            if !self.assignment.is_false(&clause[lit_index]) {
                                clause.swap(1, lit_index);
                                self.watch_lists.add_watch(!clause[1], new_watch);
                                i -= 1;
                                watch_list.remove(i);
                                continue 'watch;
                            }
                        }
                        if self.assignment.is_false(&clause[0]) {
                            return Err(Conflict::Long(clause.to_vec()));
                        }
                        let first = clause[0];
                        self.add_assign(&first, PropReason::Long(watch.cref));
                    }
                }
            }

            self.watch_lists.set_watch_list(lit, watch_list);
        }
        Ok(())
    }
}
impl Solver {
    #[inline]
    pub(crate) fn add_assign(&mut self, lit: &Lit, prop_reason: PropReason) {
        self.assignment.assign(lit);
        self.prop_queue.push_back(lit);
        self.prop_graph
            .update_node(lit, prop_reason, self.prop_queue.current_level());
    }
    pub(crate) fn backtrack(&mut self, backtrack_level: usize) {
        let new_len = self.prop_queue.level_with_trail_len[backtrack_level];
        self.prop_queue
            .level_with_trail_len
            .truncate(backtrack_level);
        self.prop_queue.pos = new_len;
        for lit in self.prop_queue.trail[new_len..].iter() {
            self.assignment.unassign(lit);
        }
        self.prop_queue.trail.truncate(new_len);
    }
}
