use crate::{lit::Lit, solver::Solver};
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
    pub(crate) fn propagate(&mut self) -> Result<()> {
        while let Some(lit) = self.prop_queue.pop_queue() {
            self.prop_binary(lit)?;
            self.prop_long(lit)?;
        }
        Ok(())
    }
    fn prop_binary(&self, lit: Lit) -> Result<()> {
        Ok(())
    }
    fn prop_long(&self, lit: Lit) -> Result<()> {
        Ok(())
    }
}
