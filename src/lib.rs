use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

pub const ORIGIN: Vec2 = Vec2 { x: 0.0, y: 0.0 };

pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };

pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };

pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };

pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };

#[wasm_bindgen]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn plus(&self, other: &Vec2) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn minus(&self, other: &Vec2) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    pub fn scale(&self, by: f32) -> Self {
        Vec2 {
            x: self.x * by,
            y: self.y * by
        }
    }

    pub fn distance_squared_to(&self, other: &Vec2) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn distance_to(&self, other: &Vec2) -> f32 {
        self
            .distance_squared_to(other)
            .sqrt()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn divide_by(&self, n: f32) -> Self {
        Vec2 {
            x: self.x / n,
            y: self.y / n
        }
    }

    pub fn normalize(&self) -> Self {
        self.divide_by(self.magnitude())
    }

    pub fn lerp(&self, towards: Self, amount: f32) -> Self {
        Vec2 {
            x: ((towards.x - self.x) * amount) + self.x,
            y: ((towards.y - self.y) * amount) + self.y
        }
    }

    pub fn limit(&self, max: f32) -> Vec2 {
        let magnitude = self.magnitude();
        if magnitude > max {
            // normalize
            self.divide_by(magnitude).scale(max)
        } else {
            self.clone()
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {

        let result = 
            Vec2::new(2.0, 10.0)
                .plus(&Vec2::new(5.0, 20.0));

        assert_eq!(result, Vec2::new(7.0, 30.0));
    }
}
