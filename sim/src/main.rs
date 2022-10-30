
use sim::{robot::{Robot, RobotInner}, pos::Pos, action::Action, simulation::Simulation, team::Team, strategy};

static ROBOT_ONE_ACCEL_RATE: f64 = 0.0;
static ROBOT_ONE_VEL_CAP: f64 = 1.0;
static ROBOT_ONE_START_POS: Pos = Pos {x: 0.0, y: 3.0};
static ROBOT_ONE_ANG_ACCEL_RATE: f64 = 0.0;
static ROBOT_ONE_ANG_VEL_CAP: f64 = 1.0;
static ROBOT_ONE_ANG: f64 = 0.0;
static ROBOT_ONE_SUBSTATION_POS: Pos = Pos {x: 0.0, y: 3.0};
static ROBOT_ONE_TEAM: Team = Team::TeamOne;
static ROBOT_ONE_TIME_TO_PICK_UP: f64 = 0.0;
static ROBOT_ONE_TIME_TO_PLACE_GROUND: f64 = 0.0;
static ROBOT_ONE_TIME_TO_PLACE_LOWER: f64 = 0.0;
static ROBOT_ONE_TIME_TO_PLACE_MIDDLE: f64 = 0.0;
static ROBOT_ONE_TIME_TO_PLACE_HIGH: f64 = 0.0;

static ROBOT_TWO_ACCEL_RATE: f64 = 0.0;
static ROBOT_TWO_VEL_CAP : f64 = 1.0;
static ROBOT_TWO_START_POS: Pos = Pos {x: 6.0, y: 3.0};
static ROBOT_TWO_ANG_ACCEL_RATE: f64 = 0.0;
static ROBOT_TWO_ANG_VEL_CAP : f64 = 1.0;
static ROBOT_TWO_ANG: f64 = std::f64::consts::PI;
static ROBOT_TWO_SUBSTATION_POS: Pos = Pos {x: 6.0, y: 3.0};
static ROBOT_TWO_TEAM: Team = Team::TeamTwo;
static ROBOT_TWO_TIME_TO_PICK_UP: f64 = 0.0;
static ROBOT_TWO_TIME_TO_PLACE_GROUND: f64 = 0.0;
static ROBOT_TWO_TIME_TO_PLACE_LOWER: f64 = 0.0;
static ROBOT_TWO_TIME_TO_PLACE_MIDDLE: f64 = 0.0;
static ROBOT_TWO_TIME_TO_PLACE_HIGH: f64 = 0.0;

static GRID_SQUARE_SIZE: f64 = 1.0;
static TIME_STEP: f64 = 1.0;

fn main() {
    let robot_one_inner = RobotInner::new(ROBOT_ONE_ACCEL_RATE, ROBOT_ONE_VEL_CAP,
    ROBOT_ONE_START_POS, ROBOT_ONE_ANG_ACCEL_RATE, ROBOT_ONE_ANG_VEL_CAP, ROBOT_ONE_ANG, 
    ROBOT_ONE_SUBSTATION_POS, ROBOT_ONE_TEAM, ROBOT_ONE_TIME_TO_PICK_UP, ROBOT_ONE_TIME_TO_PLACE_GROUND, 
    ROBOT_ONE_TIME_TO_PLACE_LOWER, ROBOT_ONE_TIME_TO_PLACE_MIDDLE, ROBOT_ONE_TIME_TO_PLACE_HIGH);
    let robot_one = Robot::new(Box::new(strategy::move_to_closest), robot_one_inner);

    let robot_two_inner = RobotInner::new(ROBOT_TWO_ACCEL_RATE, ROBOT_TWO_VEL_CAP,
        ROBOT_TWO_START_POS, ROBOT_TWO_ANG_ACCEL_RATE, ROBOT_TWO_ANG_VEL_CAP, ROBOT_TWO_ANG, 
        ROBOT_TWO_SUBSTATION_POS, ROBOT_TWO_TEAM, ROBOT_TWO_TIME_TO_PICK_UP, ROBOT_TWO_TIME_TO_PLACE_GROUND, 
        ROBOT_TWO_TIME_TO_PLACE_LOWER, ROBOT_TWO_TIME_TO_PLACE_MIDDLE, ROBOT_TWO_TIME_TO_PLACE_HIGH);
    let robot_two = Robot::new(Box::new(strategy::move_to_zero), robot_two_inner);
    let mut sim = Simulation::new_with_robots(GRID_SQUARE_SIZE, TIME_STEP, robot_one, robot_two);
    sim.run();
    println!("---------------------------------------");
    println!("Final junction state: {:#?}", sim.state().junctions);
    let scores = sim.scores();
    println!("Final scores: {} to {}", scores.0, scores.1);
}
