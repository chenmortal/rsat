use super::{lit::Lit, solver::ClauseRef};

#[derive(Debug)]
pub(super) enum Conflict {
    Binary([Lit; 2]),
    Long(usize),
}
