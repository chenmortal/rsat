use crate::{graph::PropReason, lit::Lit, solver::Solver};

impl Solver {
    pub(crate) fn make_decision(&mut self) -> bool {
        let mut index = 0;
        for ele in self.assignment.iter() {
            if ele.is_none() {
                break;
            }
            index += 1;
        }
        if index < self.assignment.len() {
            let decision = Lit::from_index(index, self.default_polarity);
            dbg!(decision);
            self.prop_queue.new_decision_level();
            self.add_assign(&decision, PropReason::Unit);
            true
        } else {
            false
        }
    }
}
