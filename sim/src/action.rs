use crate::{robot::{Robot, RobotInner}, simulation::SimState};

pub trait Action {
    fn update_time_left(&mut self, step: f64) -> bool;
    fn do_action(&mut self, r: &mut RobotInner, s: &mut SimState);
}