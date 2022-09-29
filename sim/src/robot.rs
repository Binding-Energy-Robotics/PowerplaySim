use crate::{team::Team, pos::Pos, junction::JunctionItem, simulation::Simulation, action::Action};

pub struct RobotInner {
    team: Team,
    accel_rate: f64,
    ang_accel_rate: f64,
    velocity: f64,
    ang_vel: f64,
    vel_cap: f64,
    ang_vel_cap: f64,
    angle: f64,
    pos: Pos,
    item_held: Option<JunctionItem>,
    goal_pos: Option<Pos>,
    goal_angle: Option<f64>,
    time_to_pick_up: f64,
    time_to_place_ground: f64,
    time_to_place_lower: f64,
    time_to_place_middle: f64,
    time_to_place_high: f64,
    sub_station_pos: Pos,
}

impl RobotInner {
    pub fn new(a: f64, vc: f64, p: Pos, aa: f64, avc: f64, ang: f64, 
        station: Pos, t: Team, ttpu: f64, ttpg: f64, ttpl: f64, ttpm: f64, ttph: f64) -> RobotInner {
            RobotInner { team: t, accel_rate: a, ang_accel_rate: aa, velocity: 0.0, ang_vel: 0.0, vel_cap: vc, 
                ang_vel_cap: avc, angle: ang, pos: p, item_held: None, goal_pos: None, goal_angle: None, 
                time_to_pick_up: ttpu, time_to_place_ground: ttpg, time_to_place_lower: ttpl, time_to_place_middle: ttpm, 
                time_to_place_high: ttph, sub_station_pos: station }
    }

}

pub struct Robot {
    inner: RobotInner,
    strat: Box<dyn FnMut(&mut RobotInner, &mut Simulation) -> Option<Action>>,
    action: Option<Action>,
}
 
impl Robot {
    pub fn new(s: Box<dyn FnMut(&mut RobotInner, &mut Simulation) -> Option<Action>>, r: RobotInner) -> Robot {
        Robot { inner: r, strat: s, action: None }
    }
    pub fn decide_action(&mut self, s: &mut Simulation) -> Option<Action> {
        self.strat.call_mut((&mut self.inner, s))
    }
    pub fn robot(&mut self) -> &mut RobotInner {
        &mut self.inner
    }
}


