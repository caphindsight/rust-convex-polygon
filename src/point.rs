#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn abs2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn abs(&self) -> f64 {
        self.abs2().sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn vector(a: &Point, b: &Point) -> Point {
        Point {x: b.x - a.x, y: b.y - a.y}
    }

    pub fn cross_prod(a: &Point, b: &Point) -> f64 {
        a.x * b.y - a.y * b.x
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
