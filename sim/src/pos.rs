#[derive(Clone, Copy)]
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
        let ang = f64::atan2(p.x - self.x, p.y - self.y);
        self.x += f64::cos(ang) * v * t;
        self.y += f64::cos(ang) * v * t;
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