use crate::{lit::Lit, solver::ClauseRef};
#[derive(Debug, Clone, Copy)]
pub(crate) enum PropReason {
    Unit,
    Binary([Lit;1]),
    Long(ClauseRef),
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct PropNode {
    pub(crate) reason: PropReason,
    pub(crate) level: usize,
}
#[derive(Debug, Default)]
pub(crate) struct PropGraph(Vec<PropNode>);
impl PropGraph {
    pub(crate) fn resize(&mut self, var_count: usize) {
        self.0.resize(
            var_count,
            PropNode {
                reason: PropReason::Unit,
                level: 0,
            },
        )
    }
    #[inline]
    pub(crate) fn update_node(&mut self, lit: &Lit, reason: PropReason, level: usize) {
        let node = &mut self.0[lit.index()];
        node.reason = reason;
        node.level = level;
    }
    #[inline]
    pub(crate) fn get_node(&self, lit: &Lit) -> &PropNode {
        debug_assert!(lit.index() < self.0.len());
        &self.0[lit.index()]
    }
}
