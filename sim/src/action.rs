use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use crate::{robot::{Robot, RobotInner}, simulation::SimState, junction::{JunctionItem, Junction}};

pub trait Action : Display {
    fn update_time_left(&mut self, step: f64) -> bool;
    fn do_action(self: Box<Self>, r: &mut RobotInner, s: &mut SimState);
}

pub struct PlaceItem {
    time_left: f64,
    item: JunctionItem,
    junction: Rc<Junction>,
}

impl PlaceItem {
    pub fn new(time: f64, item: JunctionItem, junction: Rc<Junction>) -> PlaceItem {
        PlaceItem { time_left: time, item, junction}
    }
}

impl Display for PlaceItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Time left: {}, item to be placed: {}", self.time_left, self.item)
    }
}

impl Action for PlaceItem {
    fn update_time_left(&mut self, step: f64) -> bool {
        self.time_left -= step;
        self.time_left <= 0.0
    }
    fn do_action(mut self: Box<Self>, r: &mut RobotInner, s: &mut SimState) {
        unsafe { Rc::get_mut_unchecked(&mut self.junction)}.add_item(self.item); 
    }
}

pub struct PickUpCone {
    time_left: f64,
}

impl PickUpCone {
    pub fn new(time: f64) -> PickUpCone {
        PickUpCone { time_left: time }
    }
}

impl Display for PickUpCone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Time left: {}", self.time_left)
    }
}

impl Action for PickUpCone {
    fn update_time_left(&mut self, step: f64) -> bool {
        self.time_left -= step;
        self.time_left <= 0.0
    }
    fn do_action(self: Box<Self>, r: &mut RobotInner, s: &mut SimState) {
        match r.get_team() {
            crate::team::Team::TeamOne => s.num_team_one_cones -= 1,
            crate::team::Team::TeamTwo => s.num_team_two_cones -= 1,
        }
        r.give_item(JunctionItem::Cone(r.get_team()));
    }
}