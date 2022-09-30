use crate::{junction::{Junction, Level}, robot::{Robot, RobotInner}, pos::Pos, action::Action, team::Team};

pub struct SimState {
    pub grid_square_size: f64,
    pub time_step: f64,
    pub junctions: Vec<Junction>,
    pub time: f64,
    pub num_team_one_cones: i8,
    pub num_team_two_cones: i8,
}

pub struct Simulation {
    robot_one: Robot,
    robot_two: Robot,
    state: SimState,
}

impl Simulation {
    pub fn new(grid_square_size: f64, time_step: f64, 
        r1ar: f64, r1vc: f64, r1aar: f64, r1avc: f64, r1ttpu: f64, r1ttpg: f64, r1ttpl: f64, r1ttpm: f64, r1ttph: f64, 
        r2ar: f64, r2vc: f64, r2aar: f64, r2avc: f64, r2ttpu: f64, r2ttpg: f64, r2ttpl: f64, r2ttpm: f64, r2ttph: f64, 
        r1s: Box<dyn FnMut(&mut RobotInner, &mut SimState) -> Option<Box<dyn Action>>>,
        r2s: Box<dyn FnMut(&mut RobotInner, &mut SimState) -> Option<Box<dyn Action>>>) -> Simulation {
        let state = SimState {
            grid_square_size, time_step,
            junctions: {
                let mut js = Vec::with_capacity(25);
                for i in 1..6 {
                    for j in 1..6 {
                        let pos = Pos::new(i as f64 * grid_square_size, j as f64 * grid_square_size);
                        let level = {
                            if (i & 2 == 1) && (j & 2 == 1) {
                                Level::Ground
                            }
                            else if (i == 1 || i == 5) && (j == 1 || j == 5) {
                                Level::Low
                            }
                            else if (i & 2 == 0) && (j & 2 == 0) {
                                Level::Middle
                            }
                            else {
                                Level::High
                            }
                        };
                        let i = i as usize;
                        let j = j as usize;
                        js[(i - 1) * 5 + j - 1] = Junction::new(pos, level);
                    }
                }
                js
            }, 
            time: 0.0, num_team_one_cones: 30, num_team_two_cones: 30
        };
        Simulation {  
            
            robot_one: Robot::new(r1s, RobotInner::new(r1ar, r1vc, Pos::new(3.0 * grid_square_size, 0.0 * grid_square_size), 
            r1aar, r1avc, 0.0, Pos::new(3.0 * grid_square_size, 0.0 * grid_square_size), Team::TeamOne, 
            r1ttpu, r1ttpg, r1ttpl, r1ttpm, r1ttph)), 
            robot_two: Robot::new(r2s, RobotInner::new(r2ar, r2vc, Pos::new(3.0 * grid_square_size, 6.0 * grid_square_size), 
            r2aar, r2avc, std::f64::consts::PI, Pos::new(3.0 * grid_square_size, 6.0 * grid_square_size), Team::TeamTwo, 
            r2ttpu, r2ttpg, r2ttpl, r2ttpm, r2ttph)), 
            state}
    }
    pub fn run(&mut self) {
        while self.state.time < 150.0 {
            self.step();
            self.state.time += self.state.time_step;
        }
    }
    pub fn step(&mut self) {
        let mut step_robot = | r: &mut Robot | {
            match &mut r.action {
                None => {
                    let a = (r.strat)(&mut r.inner, &mut self.state);
                    r.action = a;
                    // If it's still None, it means that the robot wants to move
                    if let None = &r.action {
                        r.inner.r#move(self.state.time_step);
                    }
                },
                Some(a) => {
                    if a.update_time_left(self.state.time_step) {
                        a.do_action(&mut r.inner, &mut self.state);
                    }
                }
            }
        };
        step_robot(&mut self.robot_one);
        step_robot(&mut self.robot_two);
    }
}