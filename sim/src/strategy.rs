use crate::{simulation::{Simulation, SimState}, action::Action, robot::Robot, pos::Pos};

fn move_to_zero(r: &mut Robot, _s: &mut SimState) -> Option<Box<dyn Action>> {
    r.inner.set_goal_pos(Pos::new(0.0, 0.0));
    r.inner.set_goal_angle(0.0);
    None
}

