#![feature(fn_traits)]
#![feature(get_mut_unchecked)]
#![feature(let_chains)]
pub mod simulation;
pub mod junction;
pub mod robot;
pub mod team;
pub mod pos;
pub mod action;
pub mod strategy;

#[cfg(test)]
mod tests {
    use approx_eq::assert_approx_eq;
    use rand::prelude::*;
    use super::pos::*;

    #[test]
    fn straight_distance() {  
        let mut rng = rand::thread_rng();
        for _ in 0..(rng.gen::<usize>() % 900 + 100)  { // Number between 100 - 1000
            let same = rng.gen();
            // Multiply by one hundred to spread these numbers out.
            let p1c = rng.gen::<f64>() * 100.0;
            let p2c = rng.gen::<f64>() * 100.0;
            let p1;
            let p2;
            if rand::random() {
                p1 = Pos::new(same, p1c);
                p2 = Pos::new(same, p2c);
            }
            else {
                p1 = Pos::new(p1c, same);
                p2 = Pos::new(p2c, same);
            }
            assert_eq!(p1.distance_from(p2), (p1c - p2c).abs());
        }
    }
    #[test]
    fn angle_distance() {
        let mut rng = rand::thread_rng();
        for _ in 0..(rng.gen::<usize>() % 900 + 100) {
            let ang: f64 = rng.gen();
            let leg1 = rng.gen::<f64>() * 10.0;
            let hypot = leg1 / ang.cos();
            let leg2 = ang.sin() * hypot;
            let p1 = Pos::new(0.0, 0.0);
            let p2 = Pos::new(leg1, leg2);
            assert_approx_eq!(hypot, p2.distance_from(p1));
        }
    }
    #[test]
    fn atan2() {
        let mut rng = rand::thread_rng();
        for _ in 0..(rng.gen::<usize>() % 900 + 100) {
            let ang: f64 = rng.gen();
            let leg1 = rng.gen::<f64>() * 10.0;
            let hypot = leg1 / ang.cos();
            let leg2 = ang.sin() * hypot;
            assert_approx_eq!(ang, f64::atan2(leg2, leg1));
        }
    }
    #[test]
    fn move_full() {
        let mut rng = rand::thread_rng();
        for _ in 0..(rng.gen::<usize>() % 900 + 100) {
            let mut p1 = Pos::new(rng.gen::<f64>() * 100.0, rng.gen::<f64>() * 100.0);
            let p2 = Pos::new(rng.gen::<f64>() * 100.0, rng.gen::<f64>() * 100.0);
            // This test being accurate relies on our distance function being correct.
            p1.r#move(p2, p1.distance_from(p2), 1.0);
            p1.assert_approx_eq(p2);
        }
    }
    #[test]
    fn approx_eq_reflexivity() {
        let mut rng = rand::thread_rng();
        for _ in 0..(rng.gen::<usize>() % 900 + 100) {
            let p1 = Pos::new(rng.gen::<f64>() * 100.0, rng.gen::<f64>() * 100.0);
            p1.assert_approx_eq(p1.clone());
        }
    }
}

