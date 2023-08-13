use crate::{lit::Lit, solver::ClauseRef};

#[derive(Debug)]
pub(crate) enum Conflict {
    Binary([Lit; 2]),
    Long(ClauseRef),
}
