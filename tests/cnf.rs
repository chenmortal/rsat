use anyhow::Result;
use rsat::{cnf::CnfFormula, lit::Lit, solver::Solver};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
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
                        clause.push(Lit::from_dimacs(s.abs() as usize, s > 0));
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
    let path = Path::new("tests/cnfs/sgen1_sat_90_0.cnf");
    let cnf = parse_cnf(path).unwrap();
    dbg!(Solver::new(true)
        .add_formula(&cnf)
        .solve()
        .check_satisfied());
}
#[test]
fn test_add_clause() {
    let mut formula = CnfFormula::default();
    let a = Lit::from_dimacs(1, true);
    let b = Lit::from_dimacs(2, true);
    let c = Lit::from_dimacs(3, true);
    formula.add_clause(&[a, a, b]);
    formula.add_clause(&[b, c]);
    formula.add_clause(&[!a, b]);
    formula.add_clause(&[a, !a]);
    let mut formula_iter = formula.iter();
    assert_eq!(formula_iter.next(), Some([a, b].as_ref()));
    assert_eq!(formula_iter.next(), Some([b, c].as_ref()));
    assert_eq!(formula_iter.next(), Some([!a, b].as_ref()));
    assert_eq!(formula_iter.next(), None);
}
#[test]
fn test_postive_negtive() {
    assert_eq!(Lit::from_dimacs(1, true).to_dimacs(), 1);
    assert_eq!((!Lit::from_dimacs(1, true)).to_dimacs(), 1);
}
