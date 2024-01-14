use self::vector::Op;

pub mod math_f {
    pub trait Quaternion {
        fn rotate() -> Self;
    }

    pub struct math_f;

}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x_axis: f64, y_axis: f64) -> Self {
        Vec2 { x: x_axis, y: y_axis }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}

pub mod vector {
    use super::Vec2;

    pub trait Magnitude {
        fn distance(&self, vector: Vec2) -> f64;
        fn normalize(&self) -> Vec2;
    }

    pub trait Op {
        fn add(&self, vec: Vec2) -> Vec2;
        fn sub(&self, vec: Vec2) -> Vec2;
        fn dot(&self, vec: Vec2) -> f64;
    }

    pub trait Transform {
        fn translate(&mut self, coords: Vec2);
    }
}

impl vector::Op for Vec2 {
    fn add(&self, vec: Vec2) -> Vec2 {
        Vec2 { x: self.x + vec.x, y: self.y + vec.y }
    }

    fn sub(&self, vec: Vec2) -> Vec2 {
        Vec2 { x: self.x - vec.x, y: self.y - vec.y }
    }

    fn dot(&self, vec: Vec2) -> f64 {
        self.x * vec.x + self.y * vec.y
    }
}

impl vector::Magnitude for Vec2 {
    fn distance(&self, vec: Vec2) -> f64 {
        self.sub(vec).magnitude()
    }

    fn normalize(&self) -> Vec2 {
        let mag = self.magnitude();
        Vec2 { x: self.x / mag, y: self.y / mag }
    }
}

impl vector::Transform for Vec2 {
    fn translate(&mut self, coords: Vec2) {
        self.x = coords.x;
        self.y = coords.y;
    }
}
