use std::rc::Rc;

use crate::{junction::{Junction, Level, JunctionItem}, robot::{Robot, RobotInner}, pos::Pos, action::Action, team::Team};

use rand::prelude::*;

pub struct SimState {
    pub grid_square_size: f64,
    pub time_step: f64,
    pub junctions: Vec<Rc<Junction>>,
    pub time: f64,
    pub num_team_one_cones: i8,
    pub num_team_two_cones: i8,
}

impl SimState {
    pub fn new(grid_square_size: f64, time_step: f64) -> SimState {
        SimState {
            grid_square_size, time_step,
            junctions: {
                let mut js = Vec::with_capacity(25);
                for i in 1..6 {
                    for j in 1..6 {
                        let pos = Pos::new(i as f64 * grid_square_size, j as f64 * grid_square_size);
                        let level = {
                            if (i % 2 == 1) && (j % 2 == 1) {
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
                        js.push(Rc::new(Junction::new(pos, level)));
                    }
                }
                js
            }, 
            time: 0.0, num_team_one_cones: 30, num_team_two_cones: 30
        }
    }
    pub fn can_give_cone(&self, r: &RobotInner) -> bool {
        r.get_pos().is_approx_eq(r.get_substation()) && self.has_cones_for(r.get_team())
    }
    pub fn can_place_item(&self, r: &RobotInner) -> (bool, Option<Rc<Junction>>) {
        match self.junctions.iter().filter(|j| !j.is_capped())
            .filter(|j| j.get_top_unmut().is_none() || j.get_top_unmut().as_ref().unwrap().team() != r.get_team())
            .filter(|j| j.get_pos().is_approx_eq(r.get_pos())).next() {
            Some(j) => (true, Some(Rc::clone(&j))),
            None => (false, None)
        }

    }
    pub fn has_cones_for(&self, t: Team) -> bool {
        match t {
            Team::TeamOne => self.num_team_one_cones > 0,
            Team::TeamTwo => self.num_team_two_cones > 0,
        }
    }
    pub fn junctions(&mut self) -> &mut Vec<Rc<Junction>> {
        &mut self.junctions
    }
    pub fn closest_junction(&self, r: &RobotInner) -> Rc<Junction> {
        Rc::clone(self.junctions.iter().filter(|j| match j.get_top_unmut() {
            None => true,
            Some(item) => item.team() != r.get_team(),
        }).min_by(|x, y| (x.get_pos().distance_from(r.get_pos()))
        .partial_cmp(&y.get_pos().distance_from(r.get_pos())).unwrap()).unwrap())
    }
    pub fn most_efficient_junction(&self, r: &RobotInner) -> Rc<Junction> {
        Rc::clone(self.junctions.iter().filter(|j| match j.get_top_unmut() {
            None => true,
            Some(item) => item.team() != r.get_team(),
        }).max_by(|x, y| (x.get_level().score() as f64 / x.get_pos().distance_from(r.get_pos()))
        .partial_cmp(&(y.get_level().score() as f64 / y.get_pos().distance_from(r.get_pos()))).unwrap()).unwrap())
    }
}

impl std::fmt::Display for SimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid square size: {}", self.grid_square_size)?;
        writeln!(f, "Time step: {}", self.time_step)?;
        writeln!(f, "Junctions: {:#?}", self.junctions)?;
        writeln!(f, "Time: {}", self.time)?;
        writeln!(f, "Number of team one cones remaining: {}", self.num_team_one_cones)?;
        write!(f, "Number of team two cones remaining: {}", self.num_team_two_cones)
    }
}

pub struct Simulation {
    robot_one: Robot,
    robot_two: Robot,
    state: SimState,
    rand: ThreadRng,
    scores: ((u32, (u32, u32, u32, u32)), (u32, (u32, u32, u32, u32))),
}

impl Simulation {
    pub fn new(grid_square_size: f64, time_step: f64, 
        r1ar: f64, r1vc: f64, r1aar: f64, r1avc: f64, r1ttpu: f64, r1ttpg: f64, r1ttpl: f64, r1ttpm: f64, r1ttph: f64, 
        r2ar: f64, r2vc: f64, r2aar: f64, r2avc: f64, r2ttpu: f64, r2ttpg: f64, r2ttpl: f64, r2ttpm: f64, r2ttph: f64, 
        r1s: Box<dyn FnMut(&mut RobotInner, &mut SimState) -> Option<Box<dyn Action>>>,
        r2s: Box<dyn FnMut(&mut RobotInner, &mut SimState) -> Option<Box<dyn Action>>>) -> Simulation {
        Simulation {  
            robot_one: Robot::new(r1s, RobotInner::new(r1ar, r1vc, Pos::new(3.0 * grid_square_size, 0.0 * grid_square_size), 
            r1aar, r1avc, 0.0, Pos::new(3.0 * grid_square_size, 0.0 * grid_square_size), Team::TeamOne, 
            r1ttpu, r1ttpg, r1ttpl, r1ttpm, r1ttph)), 
            robot_two: Robot::new(r2s, RobotInner::new(r2ar, r2vc, Pos::new(3.0 * grid_square_size, 6.0 * grid_square_size), 
            r2aar, r2avc, std::f64::consts::PI, Pos::new(3.0 * grid_square_size, 6.0 * grid_square_size), Team::TeamTwo, 
            r2ttpu, r2ttpg, r2ttpl, r2ttpm, r2ttph)), 
            state: SimState::new(grid_square_size, time_step), rand: thread_rng(), scores: ((0, (0,0,0,0)),(0, (0,0,0,0)))}
    }
    pub fn new_with_robots(grid_square_size: f64, time_step: f64, one: Robot, two: Robot) -> Simulation {
        Simulation { robot_one: one, robot_two: two, state: SimState::new(grid_square_size, time_step), rand: thread_rng(), 
            scores: ((0, (0,0,0,0)),(0, (0,0,0,0)))}
    }
    pub fn print_short(&self) {
        println!("robot one angle and position: {} @ {}, robot two angle and position: {} @ {},
        num cones: {}, {}", 
        self.robot_one.inner.get_angle(), self.robot_one.inner.get_pos(), 
        self.robot_two.inner.get_angle(), self.robot_two.inner.get_pos(),
        self.state.num_team_one_cones, self.state.num_team_two_cones);
        //println!("Robot one: {}", self.robot_one);
    }
    pub fn run(&mut self) {
        while self.state.time < 150.0 {
            //self.print_short();
            let before_driver = self.state.time < 30.0;
            self.step();
            self.state.time += self.state.time_step;
            if before_driver && self.state.time > 30.0 {
                //Now we;re in driver-controlled period, sum up scores.
                //self.scores = self.scores();
            }
        }
        /* 
        let driver_scores = self.scores();
        self.scores = ((self.scores.0.0 + driver_scores.0.0, (self.scores.0.1.0 + driver_scores.0.1.0, self.scores.0.1.1 + driver_scores.0.1.1, 
            self.scores.0.1.2 + driver_scores.0.1.2, self.scores.0.1.3 + driver_scores.0.1.3)), 
        (self.scores.1.0 + driver_scores.1.0, (self.scores.1.1.0 + driver_scores.1.1.0, self.scores.1.1.1 + driver_scores.1.1.1, 
            self.scores.0.1.2 + driver_scores.1.1.2, self.scores.1.1.3 + driver_scores.1.1.3)));
        */
    }
    pub fn scores(&self) -> ((u32, (u32, u32, u32, u32)), (u32, (u32, u32, u32, u32))) {
        // uhh idk scores just add
        let folder = |acc: u32, e: &Rc<Junction>, team: Team, lev: Level, points: u32| {
            acc + if let Some(item) = e.get_top_unmut() {
                match item {
                    &JunctionItem::Beacon(t) => {
                        if t == team {
                            10 // I think?
                        } else {
                            0
                        }
                    },
                    &JunctionItem::Cone(t) => {
                        if t == team && e.get_level() == &lev {
                            points
                        } else {
                            0
                        }
                    },
                }
            }
            else {
                0
            }
        };
        let t1_score_bdown = 
            (self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamOne, Level::Ground, 2)),
            self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamOne, Level::Low, 3)),
            self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamOne, Level::Middle,4)),
            self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamOne, Level::High, 5)));
        let t2_score_bdown = 
            (self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamTwo, Level::Ground, 2)),
            self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamTwo, Level::Low, 3)),
            self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamTwo, Level::Middle,4)),
            self.state.junctions.iter().fold(0,  |acc, e| folder(acc, e, Team::TeamTwo, Level::High, 5)));

        ((t1_score_bdown.0 + t1_score_bdown.1 + t1_score_bdown.2 + t1_score_bdown.3, t1_score_bdown), 
        (t2_score_bdown.0 + t2_score_bdown.1 + t2_score_bdown.2 + t2_score_bdown.3, t2_score_bdown))
    }
    pub fn step(&mut self) {
        //println!("-------------------");
        let mut step_robot = | r: &mut Robot | {
            if let None = r.action {
                let a = (r.strat)(&mut r.inner, &mut self.state);
                r.action = a;
            }
            match &mut r.action {
                None => {
                    // If it's still None, it means that the robot wants to move
                    r.inner.r#move(self.state.time_step);
                },
                Some(_a) => {
                    //println!("Robot current action: {}", _a);
                    if _a.update_time_left(self.state.time_step) {
                        //println!("Fufilling robot action");
                        let a = r.action.take().unwrap();
                        a.do_action(&mut r.inner, &mut self.state);
                        // Now that we don't have an action, we will have to decide a movement on the next step. 
                        // Therefore, we should reset the robot's goal.
                        r.inner.clear_goal();
                    }
                }
            }
        };
        if self.rand.gen() {
            step_robot(&mut self.robot_one);
            step_robot(&mut self.robot_two);
        } else {    
            step_robot(&mut self.robot_two);
            step_robot(&mut self.robot_one);
        }
    }
    pub fn state(&self) -> &SimState {
        &self.state
    }
}

impl std::fmt::Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "state: {}, robot one: {}, robot two: {}", self.state, self.robot_one, self.robot_two)
    }
}

