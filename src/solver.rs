use crate::{
    assign::Assignment,
    cdcl::AnalyzeConflict,
    db::ClauseDb,
    graph::{PropGraph, PropReason},
    lit::Lit,
    prop::PropQueue,
    watch::Watchlists,
};

use super::cnf::CnfFormula;

#[derive(Debug, Clone, Copy)]
pub(super) enum ClauseRef {
    Binary(usize),
    Long(usize),
}
#[derive(Debug)]
pub enum SolveState {
    Unknown,
    Sat,
    Unsat,
}
impl Default for SolveState {
    fn default() -> Self {
        Self::Unknown
    }
}
#[derive(Debug, Default)]
pub struct Solver {
    pub(crate) assignment: Assignment,
    pub(crate) clause_db: ClauseDb,
    max_lit_index: usize,
    pub(crate) default_polarity: bool,
    pub(crate) watch_lists: Watchlists,
    pub(crate) prop_graph: PropGraph,
    pub(crate) prop_queue: PropQueue,
    pub(crate) analyze_conflict: AnalyzeConflict,
    pub(crate) state: SolveState,
}

impl Solver {
    pub fn add_formula(mut self, formula: &CnfFormula) -> Self {
        self.max_lit_index = self.max_lit_index.max(formula.get_max_lit_index());
        let var_count = self.max_lit_index + 1;
        self.assignment.resize(var_count);
        self.watch_lists.resize(var_count);
        self.prop_graph.resize(var_count);
        self.analyze_conflict.resize(var_count);
        self.clause_db.add_formula(formula);
        
        self
    }
    pub fn new(decision_default_polarity: bool) -> Self {
        let mut solve = Solver::default();
        solve.default_polarity = decision_default_polarity;
        solve
    }
    pub fn solve(mut self) -> Self {
        self.generate_watch();
        for lit in self.clause_db.assign_clauses.clone() {
            self.add_assign(&lit, PropReason::Unit);
        }
        loop {
            match self.propagate() {
                Ok(_) => {}
                Err(conflict) => {
                    let backtrack_level = self.analyze_conflict(conflict).unwrap();
                    self.backtrack(backtrack_level);
                    let learnt_clause = self.analyze_conflict.get_learnt_clause();
                    match learnt_clause {
                        Some(ref clause) => {
                            let reason = if let Some(r) = self.clause_db.add_clause(clause) {
                                self.watch_lists.watch_clause(r, clause[0], clause[1]);
                                match r {
                                    ClauseRef::Binary(_) => PropReason::Binary([clause[1]]),
                                    ClauseRef::Long(_) => PropReason::Long(r),
                                }
                            } else {
                                PropReason::Unit
                            };
                            self.add_assign(&clause[0], reason);
                        }
                        None => {
                            self.state = SolveState::Unsat;
                            break;
                        }
                    }
                    continue;
                }
            };
            if !self.make_decision() {
                break;
            };
        }
        self.check_satisfied();
        self
    }
    pub fn check_satisfied(&mut self) -> bool {
        for bin in &self.clause_db.binary_clauses {
            if !self.assignment.is_true(&bin[0]) && !self.assignment.is_true(&bin[1]) {
                // dbg!(bin);
                self.state = SolveState::Unsat;
                return false;
            }
        }
        for clause in &self.clause_db.long_clauses {
            let mut flag = false;
            for lit in clause {
                if self.assignment.is_true(lit) {
                    flag = true;
                    break;
                }
            }
            if !flag {
                // dbg!(clause);
                self.state = SolveState::Unsat;
                return false;
            }
        }
        self.state = SolveState::Sat;
        true
    }
    pub fn get_prop_reason(&self, lit: &Lit) -> Option<Vec<Lit>> {
        match self.state {
            SolveState::Sat => match self.prop_graph.get_node(lit).reason {
                PropReason::Unit => None,
                PropReason::Binary(l) => Some(l.to_vec()),
                PropReason::Long(r) => match r {
                    ClauseRef::Binary(_) => None,
                    ClauseRef::Long(index) => {
                        let mut res = vec![];
                        for ele in &self.clause_db.long_clauses[index] {
                            if ele.index() != lit.index() {
                                res.push(!*ele);
                            }
                        }
                        Some(res)
                    }
                },
            },
            _ => None,
        }
    }
}
