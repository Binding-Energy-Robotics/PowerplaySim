
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

pub fn fn_name<T>(_ : &T) -> &'static str {
    std::any::type_name::<T>()
} 

fn main() {
    let r1_strat = strategy::move_to_most_efficient;
    let r2_strat = strategy::move_to_most_efficient_ignore_high;

    let robot_one_inner = RobotInner::new(ROBOT_ONE_ACCEL_RATE, ROBOT_ONE_VEL_CAP,
    ROBOT_ONE_START_POS, ROBOT_ONE_ANG_ACCEL_RATE, ROBOT_ONE_ANG_VEL_CAP, ROBOT_ONE_ANG, 
    ROBOT_ONE_SUBSTATION_POS, ROBOT_ONE_TEAM, ROBOT_ONE_TIME_TO_PICK_UP, ROBOT_ONE_TIME_TO_PLACE_GROUND, 
    ROBOT_ONE_TIME_TO_PLACE_LOWER, ROBOT_ONE_TIME_TO_PLACE_MIDDLE, ROBOT_ONE_TIME_TO_PLACE_HIGH);
    let robot_one = Robot::new(Box::new(r1_strat), robot_one_inner);

    let robot_two_inner = RobotInner::new(ROBOT_TWO_ACCEL_RATE, ROBOT_TWO_VEL_CAP,
        ROBOT_TWO_START_POS, ROBOT_TWO_ANG_ACCEL_RATE, ROBOT_TWO_ANG_VEL_CAP, ROBOT_TWO_ANG, 
        ROBOT_TWO_SUBSTATION_POS, ROBOT_TWO_TEAM, ROBOT_TWO_TIME_TO_PICK_UP, ROBOT_TWO_TIME_TO_PLACE_GROUND, 
        ROBOT_TWO_TIME_TO_PLACE_LOWER, ROBOT_TWO_TIME_TO_PLACE_MIDDLE, ROBOT_TWO_TIME_TO_PLACE_HIGH);
    let robot_two = Robot::new(Box::new(r2_strat), robot_two_inner);
    let mut sim = Simulation::new_with_robots(GRID_SQUARE_SIZE, TIME_STEP, robot_one, robot_two);
    
    sim.run();
    println!("Chosen strategies: {} vs {}",
    fn_name(&r1_strat), fn_name(&r2_strat));
    println!("Final junction state: {:#?}", sim.state().junctions);
    let scores = sim.scores();
    println!("Final scores:\n{} ({}g, {}l, {}m, {}h)\n  to\n{} ({}g, {}l, {}m, {}h)", 
        scores.0.0, scores.0.1.0, scores.0.1.1, scores.0.1.2, scores.0.1.3, 
        scores.1.0, scores.1.1.0, scores.1.1.1, scores.1.1.2, scores.1.1.3,);

}
