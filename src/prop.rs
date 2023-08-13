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
    every_decision_level_len: Vec<usize>,
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
    fn clear(&mut self) {
        self.trail.clear();
        self.pos = 0;
    }
    pub(crate) fn new_decision_level(&mut self) {
        self.every_decision_level_len
            .push(self.trail.len() - self.every_decision_level_len.last().unwrap_or(&0))
    }
    pub(crate) fn current_level(&self) -> usize {
        self.every_decision_level_len.len()
    }
}
impl From<&[Lit]> for PropQueue {
    fn from(value: &[Lit]) -> Self {
        PropQueue {
            trail: value.to_vec(),
            pos: 0,
            every_decision_level_len: vec![],
        }
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
                            return Err(Conflict::Long(watch.cref));
                        }
                        let first = clause[0];
                        self.add_assign(&first, PropReason::Long(watch.cref));
                    }
                }
            }
            'watch: for i in 0..watch_list.len() {
                let watch = watch_list.get_mut(i).unwrap();
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
                                self.watch_lists.add_watch(!clause[lit_index], new_watch);
                                watch_list.remove(i);
                                continue 'watch;
                            }
                        }
                        if self.assignment.is_false(&clause[0]) {
                            return Err(Conflict::Long(watch.cref));
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
