use crate::{simulation::{Simulation, SimState}, action::Action, robot::{Robot, RobotInner}, pos::Pos};

pub fn move_to_zero(r: &mut RobotInner, _s: &mut SimState) -> Option<Box<dyn Action>> {
    r.set_goal_pos(Pos::new(0.0, 0.0));
    r.set_goal_angle(0.0);
    None
}

