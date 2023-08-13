use std::ops::Deref;

use crate::{graph::PropReason, lit::Lit, solver::Solver};

#[derive(Debug, Default)]
pub(crate) struct Assignment {
    assignment: Vec<Option<bool>>,
}
impl Assignment {
    #[inline]
    pub(crate) fn assign(&mut self, lit: &Lit) {
        self.assignment[lit.index()] = lit.is_positive().into();
    }
    #[inline]
    pub(crate) fn unassign(&mut self, lit: &Lit) {
        self.assignment[lit.index()] = None;
    }
    #[inline]
    pub(crate) fn resize(&mut self, var_count: usize) {
        self.assignment.resize(var_count, None);
    }
    #[inline]
    pub(crate) fn value(&self, lit: &Lit) -> Option<bool> {
        match self.assignment[lit.index()] {
            Some(assign) => Some(assign == lit.is_positive()),
            None => None,
        }
    }
    #[inline]
    pub(crate) fn is_true(&self, lit: &Lit) -> bool {
        self.value(lit) == Some(true)
    }
    #[inline]
    pub(crate) fn is_false(&self, lit: &Lit) -> bool {
        self.value(lit) == Some(false)
    }
    #[inline]
    pub(crate) fn is_none(&self, lit: &Lit) -> bool {
        self.value(lit) == None
    }
}
impl Deref for Assignment {
    type Target = Vec<Option<bool>>;

    fn deref(&self) -> &Self::Target {
        &self.assignment
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
}
