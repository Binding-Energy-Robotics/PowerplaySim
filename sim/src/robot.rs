use crate::{team::Team, pos::Pos, junction::{JunctionItem, Junction, Level}, simulation::{Simulation, SimState}, action::Action};

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
    pub fn set_goal_pos(&mut self, p: Pos) {
        self.goal_pos = Some(p);
    }
    pub fn set_goal_angle(&mut self, ang: f64) {
        self.goal_angle = Some(ang);
    }
    pub fn r#move(&mut self, timestep: f64) {
        // For now we don't actually care about acceleration
        // This is just a hack until we implement acceleration.
        self.velocity = self.vel_cap;
        self.ang_vel = self.ang_vel_cap;
        // First, change the angle
        if self.angle < self.goal_angle.unwrap() {
            if self.goal_angle.unwrap() - self.angle < self.ang_vel_cap {
                self.angle = self.goal_angle.unwrap();
            }
            else {
                self.angle += self.velocity;
            }
        }
        else {
            if self.angle - self.goal_angle.unwrap() < self.ang_vel_cap {
                self.angle = self.goal_angle.unwrap();
            }
            else {
                self.angle -= self.velocity;
            }
        }
        if self.goal_pos.unwrap().distance_from(self.pos) < self.velocity {
            self.pos = self.goal_pos.unwrap();
        }
        else {
            self.pos.r#move(self.goal_pos.unwrap(), self.velocity, timestep);
        }
    }
    pub fn get_pos(&self) -> Pos {
        self.pos
    }
    pub fn get_angle(&self) -> f64 {
        self.angle
    }
    pub fn get_team(&self) -> Team {
        self.team
    }
    pub fn give_item(&mut self, item: JunctionItem) {
        self.item_held = Some(item);
    }
    pub fn get_item(&self) -> &Option<JunctionItem> {
        &self.item_held
    }
    pub fn take_item(&mut self) -> Option<JunctionItem> {
        self.item_held.take()
    }
    pub fn get_substation(&self) -> Pos {
        self.sub_station_pos
    }
    pub fn get_time_pick_up(&self) -> f64 {
        self.time_to_pick_up
    }
    pub fn get_time(&self, l: Level) -> f64 {
        match l {
            Level::Ground => self.time_to_place_ground,
            Level::Low => self.time_to_place_lower,
            Level::Middle => self.time_to_place_middle,
            Level::High => self.time_to_place_high,
        }
    }
}

impl std::fmt::Display for RobotInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Team: {}", self.team)?;
        writeln!(f, "Acceleration rate: {}", self.accel_rate)?;
        writeln!(f, "Angular acceleration rate: {}", self.ang_accel_rate)?;
        writeln!(f, "Velocity: {}", self.velocity)?;
        writeln!(f, "Angular velocity: {}", self.ang_vel)?;
        writeln!(f, "Velocity cap: {}", self.vel_cap)?;
        writeln!(f, "Angular velocity cap: {}", self.ang_vel_cap)?;
        writeln!(f, "Angle: {}", self.angle)?;
        writeln!(f, "Position: {}", self.pos)?;
        match &self.item_held {
            None => writeln!(f, "This robot is currently not holding an item")?,
            Some(i) => writeln!(f, "Held item: {}", i)?
        };
        match self.goal_pos {
            None => writeln!(f, "This robot currently has no goal position")?,
            Some(p) => writeln!(f, "Goal position: {}", p)?
        };
        match self.goal_angle {
            None => writeln!(f, "This robot currently has no goal angle")?,
            Some(a) => writeln!(f, "Goal angle: {}", a)?
        };
        writeln!(f, "Time to pick up an item: {}", self.time_to_pick_up)?;
        writeln!(f, "Time to place on ground junction: {}", self.time_to_place_ground)?;
        writeln!(f, "Time to place on lower junction: {}", self.time_to_place_lower)?;
        writeln!(f, "Time to place on middle junction: {}", self.time_to_place_middle)?;
        writeln!(f, "Time to place on junction junction: {}", self.time_to_place_high)?;
        write!(f, "Substation position: {}", self.sub_station_pos)
    }
}

pub struct Robot {
    pub inner: RobotInner,
    pub strat: Box<dyn FnMut(&mut RobotInner, &mut SimState) -> Option<Box<dyn Action>>>,
    pub action: Option<Box<dyn Action>>,
}
 
impl Robot {
    pub fn new(s: Box<dyn FnMut(&mut RobotInner, &mut SimState) -> Option<Box<dyn Action>>>, r: RobotInner) -> Robot {
        Robot { inner: r, strat: s, action: None }
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.inner)?;
        match &self.action {
            None => write!(f, "This robot currently has no action"),
            Some(a) => write!(f, "Current action: {}", a)
        }
        
    }
}


