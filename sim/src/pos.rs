use approx_eq::assert_approx_eq;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Pos {
        Pos { x, y }
    }
    pub fn distance_from(&self, p: Pos) -> f64 {
        f64::sqrt(f64::powi(p.x - self.x, 2) + f64::powi(p.y - self.y, 2))
    }
    pub fn r#move(&mut self, p: Pos, v: f64, t: f64) {
        let ang = f64::atan2(p.y - self.y, p.x - self.x);
        self.x += f64::cos(ang) * v * t;
        self.y += f64::sin(ang) * v * t;
    }
    pub fn assert_approx_eq(&self, other: Pos) {
        assert_approx_eq!(self.x, other.x);
        assert_approx_eq!(self.y, other.y);
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}