use std::{
    f32::consts::PI,
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn snapped_vector_15deg(&self) -> Vec2D {
        let current_angle = (self.y / self.x).atan();
        let current_norm2 = self.norm2();
        let new_angle = (current_angle / 0.261_799_4).round() * 0.261_799_4;

        let (a, b) = if new_angle.abs() < PI / 4.0
        // 45°
        {
            let b = (current_norm2 / ((PI / 2.0 - new_angle).tan().powi(2) + 1.0)).sqrt();
            let a = (current_norm2 - b * b).sqrt();
            (a, b)
        } else {
            let a = (current_norm2 / (new_angle.tan().powi(2) + 1.0)).sqrt();
            let b = (current_norm2 - a * a).sqrt();
            (a, b)
        };

        if self.x >= 0.0 && self.y >= 0.0 {
            Vec2D::new(a, b)
        } else if self.x < 0.0 && self.y >= 0.0 {
            Vec2D::new(-a, b)
        } else if self.x >= 0.0 && self.y < 0.0 {
            Vec2D::new(a, -b)
        } else {
            Vec2D::new(-a, -b)
        }
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2D::new(self.x * rhs, self.y * rhs)
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn rect_ensure_positive_size(pos: Vec2D, size: Vec2D) -> (Vec2D, Vec2D) {
    let (pos_x, size_x) = if size.x > 0.0 {
        (pos.x, size.x)
    } else {
        ((pos.x + size.x), size.x.abs())
    };

    let (pos_y, size_y) = if size.y > 0.0 {
        (pos.y, size.y)
    } else {
        ((pos.y + size.y), size.y.abs())
    };

    (Vec2D::new(pos_x, pos_y), Vec2D::new(size_x, size_y))
}
