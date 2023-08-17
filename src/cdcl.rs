use std::mem::replace;

use crate::{
    graph::{PropGraph, PropReason},
    lit::Lit,
    prop::PropQueue,
    solver::{ClauseRef, Solver},
};
use anyhow::Result;
#[derive(Debug)]
pub(crate) enum Conflict {
    Binary([Lit; 2]),
    Long(Vec<Lit>),
}
#[derive(Debug, Default)]
pub(crate) struct AnalyzeConflict {
    seen: Vec<bool>,
    to_search_node: usize,
    learnt_clause: Vec<Lit>,
}

impl AnalyzeConflict {
    pub(crate) fn resize(&mut self, var_count: usize) {
        self.seen.resize(var_count, false);
    }
    pub(crate) fn get_learnt_clause(&mut self) -> Option<Vec<Lit>> {
        if self.learnt_clause.len() == 0 {
            None
        } else {
            Some(replace(&mut self.learnt_clause, vec![]))
        }
    }
}
impl Solver {
    pub(crate) fn analyze_conflict(&mut self, conflict: Conflict) -> Result<usize> {
        let clause = match conflict {
            Conflict::Binary(ref v) => v,
            Conflict::Long(ref v) => v.as_slice(),
        };
        let seen = &mut self.analyze_conflict.seen;
        let to_search_node = &mut self.analyze_conflict.to_search_node;
        *to_search_node = 0;
        let learnt_clause = &mut self.analyze_conflict.learnt_clause;
        learnt_clause.clear();
        AnalyzeConflict::analyze_clause(
            &self.prop_graph,
            &self.prop_queue,
            clause,
            seen,
            to_search_node,
            learnt_clause,
        );
        for lit in self.prop_queue.iter().rev() {
            if seen[lit.index()] {
                seen[lit.index()] = false;
                if *to_search_node == 1 {
                    learnt_clause.push(!*lit);
                    let last = learnt_clause.len() - 1;
                    learnt_clause.swap(0, last);
                    break;
                } else {
                    let clause: &[Lit] = match self.prop_graph.get_node(lit).reason {
                        PropReason::Unit => &[],
                        PropReason::Binary(ref block) => block,
                        PropReason::Long(clause_ref) => match clause_ref {
                            ClauseRef::Binary(_) => &[],
                            ClauseRef::Long(cref) => &self.clause_db.long_clauses[cref][1..],
                        },
                    };
                    AnalyzeConflict::analyze_clause(
                        &self.prop_graph,
                        &self.prop_queue,
                        clause,
                        seen,
                        to_search_node,
                        learnt_clause,
                    );
                }
                *to_search_node -= 1;
            }
        }
        let mut backtrack = 0;
        if learnt_clause.len() >= 2 {
            let sec = &learnt_clause[1];
            backtrack = self.prop_graph.get_node(sec).level;
            for i in 2..learnt_clause.len() {
                let level = self.prop_graph.get_node(&learnt_clause[i]).level;
                if level > backtrack {
                    backtrack = level;
                    learnt_clause.swap(1, i);
                }
            }
        }
        Ok(backtrack)
    }
}
impl AnalyzeConflict {
    fn analyze_clause(
        prop_graph: &PropGraph,
        prop_queue: &PropQueue,
        clause: &[Lit],
        seen: &mut Vec<bool>,
        to_search_node: &mut usize,
        learnt_clause: &mut Vec<Lit>,
    ) {
        for lit in clause {
            let node = prop_graph.get_node(&lit);
            if !seen[lit.index()] && node.level > 0 {
                seen[lit.index()] = true;
                if node.level == prop_queue.current_level() {
                    *to_search_node += 1;
                } else {
                    learnt_clause.push(*lit);
                }
            }
        }
    }
}
