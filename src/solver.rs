use crate::{
    assign::Assignment,
    db::ClauseDb,
    graph::{PropGraph, PropReason},
    prop::PropQueue,
    watch::Watchlists,
};

use super::cnf::CnfFormula;

#[derive(Debug, Clone, Copy)]
pub(super) enum ClauseRef {
    Binary(usize),
    Long(usize),
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
}

impl Solver {
    pub fn add_formula(mut self, formula: &CnfFormula) -> Self {
        self.max_lit_index = self.max_lit_index.max(formula.get_max_lit_index());
        let var_count = self.max_lit_index + 1;
        self.assignment.resize(var_count);
        self.watch_lists.resize(var_count);
        self.prop_graph.resize(var_count);
        self.clause_db.add_formula(formula);
        self
    }
    pub fn new(decision_default_polarity: bool) -> Self {
        let mut solve = Solver::default();
        solve.default_polarity = decision_default_polarity;
        solve
    }
    pub fn solve(&mut self) {
        self.generate_watch();
        for lit in self.clause_db.assign_clauses.clone() {
            self.add_assign(&lit, PropReason::Unit);
        }
        loop {
            match self.propagate() {
                Ok(_) => {}
                Err(e) => {
                    dbg!(e);
                }
            };
            if !self.make_decision() {
                break;
            };
        }
    }
}
// impl Solver {
//     // pub(crate) fn new(&mut self,formula:&CnfFormula){
//     //     formula.get_lit_count()

//     // }
//     pub fn add_formula(&mut self, formula: &CnfFormula) {

//     }
// pub fn solve(&mut self) -> Result<Vec<Lit>> {
// self.generate_watch();
// for clause in &self.assign_clauses.clone() {
//     let lit = &clause.0;
//     self.prop_queue.push_back(lit);
//     self.assignment.assign(lit);
//     // for clause_ref in self.watchlists[lit.code()].list.iter() {
//     // match clause_ref {
//     //     ClauseRef::Binary(offset) => {
//     //         self.binary_clauses[*offset]
//     //             .as_mut()
//     //             .check_satisfied(&self.assignment);
//     //     }
//     //     ClauseRef::Long(offset) => {
//     //         self.long_clauses[*offset]
//     //             .as_mut()
//     //             .check_satisfied(&self.assignment);
//     //     }
//     // }
//     // }
// }
// if self.prop_queue.trail.is_empty() {
//     self.prop_history.new_level();
// } else {
//     match self.propagate() {
//         Ok(_) => {}
//         Err(s) => {
//             dbg!(s);
//         }
//     };
// }
// match self.propagate() {
//     Ok(_) => {}
//     Err(s) => {
//         dbg!(s);
//     }
// };
// // while self.make_decision()? {
// //     // self.propagate()
// //     if self.propagate().is_err() {
// //         self.handle_conflict()?;
// //     };
// // }
// let mut res = Vec::new();
// for index in 0..self.assignment.len() {
//     match self.assignment[index] {
//         Some(b) => {
//             let lit = Lit::from_index(index);
//             res.push(if b { lit } else { !lit });
//         }
//         None => {}
//     }
// }
// Ok(res)
// }
// fn handle_conflict(&mut self) -> Result<()> {
//     self.prop_queue.clear();
//     if self.prop_history.get_level() == 1 || self.decisions.len() == 0 {
//         bail!("exist conflict");
//     }
//     let (clause_ref, index) = self.decisions.pop().unwrap();
//     let unassign_lit = match clause_ref {
//         ClauseRef::Binary(offset) => self.binary_clauses[offset].as_mut()[index],
//         ClauseRef::Long(offset) => self.long_clauses[offset].as_mut()[index],
//     };
//     let mut unassign_vec = self.prop_history.pop().unwrap();
//     unassign_vec.push(unassign_lit);
//     // for lit in unassign_vec {
//     //     self.assignment.unassign(&lit);
//     //     for clause_ref in self.watchlists[lit.code()].list.iter() {
//     //         match clause_ref {
//     //             ClauseRef::Binary(offset) => {
//     //                 self.binary_clauses[*offset]
//     //                     .as_mut()
//     //                     .check_satisfied(&self.assignment);
//     //             }
//     //             ClauseRef::Long(offset) => {
//     //                 self.long_clauses[*offset]
//     //                     .as_mut()
//     //                     .check_satisfied(&self.assignment);
//     //             }
//     //         }
//     //     }
//     // }
//     // match clause_ref {
//     //     ClauseRef::Binary(offset) => {
//     //         let binary_clause = self.binary_clauses[offset].as_mut();
//     //         let pre_dec = binary_clause[index];
//     //         self.assignment.unassign(&pre_dec);
//     //         if index == 1 {
//     //             self.handle_conflict()?;
//     //         } else {
//     //             let lit = binary_clause.clause[1];
//     //             binary_clause.state = Some(true);
//     //             self.prop_queue.push_back(&lit);
//     //             self.decisions.push((ClauseRef::Binary(offset), 1));
//     //             match self.propagate() {
//     //                 Ok(_) => {}
//     //                 Err(_) => {
//     //                     self.handle_conflict()?;
//     //                 }
//     //             }
//     //             return Ok(());
//     //         }
//     //     }
//     //     ClauseRef::Long(offset) => {
//     //         let long_clause = self.long_clauses[offset].as_mut();
//     //         for i in (index + 1)..long_clause.clause.len() {
//     //             let lit = long_clause[i];
//     //             if self.assignment.assign_bool(&lit).is_none() {
//     //                 self.assignment.assign(&lit);
//     //                 long_clause.state = Some(true);
//     //                 self.prop_queue.push_back(&lit);
//     //                 self.decisions.push((ClauseRef::Long(offset), i));
//     //                 match self.propagate() {
//     //                     Ok(_) => {}
//     //                     Err(_) => {
//     //                         self.handle_conflict()?;
//     //                     }
//     //                 }
//     //                 return Ok(());
//     //             };
//     //         }
//     //         self.handle_conflict()?;
//     //     }
//     // }
//     Ok(())
// }
// // fn propagate(&mut self) -> Result<(), Conflict> {
//     self.prop_history.new_level();
//     while let Some(lit) = self.prop_queue.pop_queue() {
//         for clause_ref in self.watchlists[(!lit).code()].list.iter() {
//             match clause_ref {
//                 ClauseRef::Binary(bin_ref) => {
//                     let binary_clause = self.binary_clauses[*bin_ref].as_mut();
//                     let [m, n] = binary_clause.clause;
//                     let other_lit = if m == !lit { n } else { m };
//                     match self.assignment.assign_bool(&other_lit) {
//                         Some(true) => {}
//                         Some(false) => return Err(Conflict::Binary([m, n])),
//                         None => {
//                             self.assignment.assign(&other_lit);
//                             binary_clause.state = Some(true);
//                             self.prop_history.add_lit(&other_lit);
//                             self.prop_queue.push_back(&other_lit);
//                         }
//                     };
//                 }
//                 ClauseRef::Long(long_ref) => {
//                     let long_clause = self.long_clauses[*long_ref].as_mut();
//                     if long_clause.state != Some(true) {
//                         let mut no_assign = Vec::new();
//                         let mut clause_flag = false;
//                         for other_lit in long_clause.iter().filter(|l| **l != !lit) {
//                             match self.assignment.assign_bool(other_lit) {
//                                 Some(true) => {
//                                     clause_flag = true;
//                                     break;
//                                 }
//                                 Some(false) => {}
//                                 None => {
//                                     no_assign.push(*other_lit);
//                                 }
//                             };
//                         }
//                         if !clause_flag {
//                             match no_assign.len().cmp(&1) {
//                                 Ordering::Less => {
//                                     return Err(Conflict::Long(*long_ref));
//                                 }
//                                 Ordering::Equal => {
//                                     let rest_lit = no_assign[0];
//                                     self.assignment.assign(&rest_lit);
//                                     long_clause.state = Some(true);
//                                     self.prop_history.add_lit(&rest_lit);
//                                     self.prop_queue.push_back(&rest_lit);
//                                 }
//                                 Ordering::Greater => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     self.prop_queue.clear();
//     Ok(())
// }
// fn make_decision(&mut self) -> Result<bool> {
//     self.de += 1;
//     let mut flag = true;
//     if self.decisions.len() != 0 {
//         if self.decisions[0].1 != 0 {
//             flag = false;
//         }
//     }

//     // if self.de == 157544 {
//     //     // dbg!(self.decisions.len());
//     //     flag = false;
//     //     // let a=self.decisions.last();
//     // }
//     for i in 0..self.long_clauses.len() {
//         let long_clause = self.long_clauses[i].as_mut();
//         if !flag && i == 70 {
//             flag = true;
//         }
//         match long_clause.check_satisfied(&self.assignment) {
//             Some(true) => {}
//             Some(false) => {
//                 self.handle_conflict()?;
//                 // return Ok(true);
//             }
//             None => {
//                 for l in 0..long_clause.len() {
//                     let lit = long_clause[l];
//                     if self.assignment.assign_bool(&lit).is_none() {
//                         self.assignment.assign(&lit);
//                         long_clause.state = Some(true);
//                         self.prop_queue.push_back(&lit);
//                         self.decisions.push((ClauseRef::Long(i), l));
//                         return Ok(true);
//                     };
//                 }
//             }
//         }
//     }
//     for i in 0..self.binary_clauses.len() {
//         let binary_clause = self.binary_clauses[i].as_mut();
//         if binary_clause.check_satisfied(&self.assignment) != Some(true) {
//             let (lit, offset) = if self.assignment.assign_bool(&binary_clause[0]).is_none() {
//                 (binary_clause[0], 0)
//             } else {
//                 (binary_clause[1], 1)
//             };
//             self.assignment.assign(&lit);
//             binary_clause.state = Some(true);
//             self.prop_queue.push_back(&lit);
//             self.decisions.push((ClauseRef::Binary(i), offset));
//             return Ok(true);
//         }
//     }
//     Ok(!self.check_all_clauses_satisfied())
// }
// #[inline]
// fn check_all_clauses_satisfied(&self) -> bool {
//     self.binary_clauses.iter().all(|x| x.state == Some(true))
//         && self.long_clauses.iter().all(|x| x.state == Some(true))
// }
// fn generate_watch(&mut self) {
//     self.watchlists
//         .resize_with(self.lit_count * 2, Watchlist::default);
//     for i in 0..self.binary_clauses.len() {
//         for lit in self.binary_clauses[i].iter() {
//             self.watchlists[lit.code()].add_ref(ClauseRef::Binary(i));
//         }
//     }
//     for i in 0..self.long_clauses.len() {
//         for lit in self.long_clauses[i].iter() {
//             self.watchlists[lit.code()].add_ref(ClauseRef::Long(i));
//         }
//     }
// }

// impl Watchlist {
//     fn update_clause(&self, lit: &Lit, binary: &mut Vec<BinaryClause>, long: &mut Vec<LongClause>) {
//         for clause_ref in self.list.iter() {
//             match clause_ref {
//                 ClauseRef::Binary(offset) => {
//                     let clause = binary[*offset].as_mut();
//                 }
//                 ClauseRef::Long(offset) => {
//                     let clause = long[*offset].as_mut();
//                 }
//             }
//         }
//     }
// }
// #[derive(Debug, Default)]
// pub(super) struct PropQueue {
//     trail: Vec<Lit>,
//     pos: usize,
// }
// impl PropQueue {
//     pub(super) fn pop_queue(&mut self) -> Option<Lit> {
//         if let Some(s) = self.trail.get(self.pos) {
//             self.pos += 1;
//             Some(*s)
//         } else {
//             None
//         }
//     }
//     pub(super) fn push_back(&mut self, lit: &Lit) {
//         self.trail.push(*lit);
//     }
//     fn clear(&mut self) {
//         self.trail.clear();
//         self.pos = 0;
//     }
// }
// impl From<&[Lit]> for PropQueue {
//     fn from(value: &[Lit]) -> Self {
//         PropQueue {
//             trail: value.to_vec(),
//             pos: 0,
//         }
//     }
// }
// #[derive(Debug, Default)]
// struct PropHistory {
//     history: Vec<Vec<Lit>>,
//     last_level: usize,
// }

// impl PropHistory {
//     fn add_lit(&mut self, lit: &Lit) {
//         let last_level = self.history.len() - 1;
//         self.history[last_level].push(*lit);
//     }
//     fn get_level(&self) -> usize {
//         self.history.len()
//     }
//     fn new_level(&mut self) {
//         self.last_level += 1;
//         self.history.push(Vec::new());
//     }
//     fn pop(&mut self) -> Option<Vec<Lit>> {
//         // self.last_level-=1;
//         let r = self.history.pop();

//         r
//     }
//     fn pop_with_level(&mut self, level: usize) -> Option<Vec<Lit>> {
//         if level <= self.last_level {
//             let p = self.history[level].clone();
//             self.history.remove(level);

//             Some(p)
//         } else {
//             None
//         }
//     }
// }
