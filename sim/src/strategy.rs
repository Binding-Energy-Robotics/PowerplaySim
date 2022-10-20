use crate::{simulation::{Simulation, SimState}, action::{Action, PickUpCone, PlaceItem}, robot::{Robot, RobotInner}, pos::Pos, junction::*};

pub fn move_to_zero(r: &mut RobotInner, _s: &mut SimState) -> Option<Box<dyn Action>> {
    r.set_goal_pos(Pos::new(0.0, 0.0));
    r.set_goal_angle(0.0);
    None
}
pub fn move_to_closest(r: &mut RobotInner, s: &mut SimState) -> Option<Box<dyn Action>> {
    if let None = r.get_item() {
        if s.can_give_cone(r) {
            Some(Box::new(PickUpCone::new(r.get_time_pick_up())))
        }
        else {
            r.set_goal_pos(r.get_substation());
            None
        }   
    }
    else {
        let (can_place, junc) = s.can_place_item(r);
        if can_place {
            let time = r.get_time(*junc.as_ref().unwrap().get_level()) ;
            Some(Box::new(PlaceItem::new(time, r.take_item().unwrap(),junc.unwrap())))
        }
        else {
            let j = s.closest_junction(&r);
            r.set_goal_pos(*j.get_pos());
            r.set_goal_angle(r.get_pos().angle_to(*j.get_pos()));
            None
        }
    }
}

