use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// use crate::sat::{cnf::CnfFormula, lit::Lit};

use crate::{cnf::CnfFormula, lit::Lit};

use super::{solver::{PropQueue, Solver}, lit};
use anyhow::Result;
fn parse_cnf<P: AsRef<Path>>(path: P) -> Result<CnfFormula> {
    let mut lines = BufReader::new(File::open(path)?).lines();

    lines.next();
    let mut cnf = CnfFormula::default();
    for line in lines {
        let mut clause = Vec::new();
        if let Ok(c) = line {
            for ele in c.split_whitespace() {
                if let Ok(s) = ele.parse::<isize>() {
                    if s != 0 {
                        let lit = Lit::from_dimacs(s.unsigned_abs());
                        // let lita = Lit::from_dimacs(1);
                        // clause.push(if s < 0 { !lit } else { lit });
                        // let lit = Lit::from_dimacs(1);;
                    }
                }
            }
        }
        cnf.add_clause(clause.as_slice());
    }
    Ok(cnf)
}
#[test]
fn test_cnf() {
    if let Ok(cnf) = parse_cnf("./tests/cnfs/sgen1_sat_90_0.cnf") {
        let mut solver = Solver::default();
        solver.add_formula(&cnf);
        match solver.solve() {
            Ok(s) => {
                let model: HashSet<Lit> = s.iter().map(|x| *x).collect();
                for clause in cnf.iter() {
                    let mut v = Vec::new();
                    for lit in clause.iter() {
                        if model.contains(lit) {
                            v.push(lit);
                        };
                    }
                    println!("{:?} \t {:?} ", clause, v);
                    clause.iter().any(|x| model.contains(x));
                }
            }
            Err(e) => {
                dbg!(e);
            }
        }
    };
}
#[test]
fn test_cnf_unsat() {
    if let Ok(cnf) = parse_cnf("./tests/cnfs/sgen1_unsat_57_0.cnf") {
        let mut solver = Solver::default();
        solver.add_formula(&cnf);
        match solver.solve() {
            Ok(s) => {
                let model: HashSet<Lit> = s.iter().map(|x| *x).collect();
                for clause in cnf.iter() {
                    let mut v = Vec::new();
                    for lit in clause.iter() {
                        if model.contains(lit) {
                            v.push(lit);
                        };
                    }
                    println!("{:?} \t {:?} ", clause, v);
                    // clause.iter().any(|x| model.contains(x));
                }
            }
            Err(e) => {
                dbg!(e);
            }
        }
    };
}
#[test]
fn test_clause() {
    // let a = Lit::from_dimacs(1);
    // let b = Lit::from_dimacs(2);
    // let c = Lit::from_dimacs(3);
    // let assignment = vec![Some(true), Some(false), None];
    // assert_eq!(
    //     {
    //         let mut c = Clause::new(&[a]).unwrap();
    //         c.check_satisfied(&assignment);
    //         c.is_satisfied()
    //     },
    //     Some(true)
    // );
    // assert_eq!(
    //     {
    //         let mut c = Clause::new(&[a, b]).unwrap();
    //         c.check_satisfied(&assignment);
    //         c.is_satisfied()
    //     },
    //     Some(true)
    // );
    // assert_eq!(
    //     {
    //         let mut c = Clause::new(&[!a, b]).unwrap();
    //         c.check_satisfied(&assignment);
    //         c.is_satisfied()
    //     },
    //     Some(false)
    // );
    // assert_eq!(
    //     {
    //         let mut c = Clause::new(&[a, b, !c]).unwrap();
    //         c.check_satisfied(&assignment);
    //         c.is_satisfied()
    //     },
    //     Some(true)
    // );
    // assert_eq!(
    //     {
    //         let mut c = Clause::new(&[!a, c]).unwrap();
    //         c.check_satisfied(&assignment);
    //         c.is_satisfied()
    //     },
    //     None
    // )
}
#[test]
fn test_queue() {
    let mut prop_queue = PropQueue::default();
    prop_queue.push_back(&Lit::from_dimacs(1));
    prop_queue.push_back(&Lit::from_dimacs(2));
    while let Some(s) = prop_queue.pop_queue() {
        if s.to_dimacs() == 1 {
            prop_queue.push_back(&Lit::from_dimacs(5));
        }
        dbg!(s);
    }
}
#[test]
fn test_solve_exp1() {
    let a = Lit::from_dimacs(1);
    let b = Lit::from_dimacs(2);
    let c = Lit::from_dimacs(3);
    let d = Lit::from_dimacs(4);
    let mut formula = CnfFormula::default();
    formula.add_clause(&[a]);
    formula.add_clause(&[!a, b]);
    formula.add_clause(&[!a, !b, c, d]);
    formula.add_clause(&[!a, !b, c]);
    let mut solver = Solver::default();
    solver.add_formula(&formula);
    assert_eq!(solver.solve().unwrap(), vec![a, b, c, d]);
}
#[test]
fn test_solve_exp2() {
    let a = Lit::from_dimacs(1);
    let b = Lit::from_dimacs(2);
    let c = Lit::from_dimacs(3);
    let d = Lit::from_dimacs(4);
    let e = Lit::from_dimacs(5);
    let mut formula = CnfFormula::default();
    formula.add_clause(&[a]);
    formula.add_clause(&[!a, b, c]);
    formula.add_clause(&[!a, d, e]);
    formula.add_clause(&[!b, !d]);
    let mut solver = Solver::default();
    solver.add_formula(&formula);
    assert_eq!(solver.solve().unwrap(), vec![a, b, !d, e]);
}
#[test]
fn test_solve_exp3() {
    let a = Lit::from_dimacs(1);
    let b = Lit::from_dimacs(2);
    let c = Lit::from_dimacs(3);
    let d = Lit::from_dimacs(4);
    let e = Lit::from_dimacs(5);
    let mut formula = CnfFormula::default();
    formula.add_clause(&[a]);
    formula.add_clause(&[!a, b, c]);
    formula.add_clause(&[!d, !e]);
    let mut solver = Solver::default();
    solver.add_formula(&formula);
    assert_eq!(solver.solve().unwrap(), vec![a, b, !d]);
}
