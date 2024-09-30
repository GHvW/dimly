#[derive(PartialEq, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32
}

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

    pub fn distance_to(&self, other: &Vec2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let result = 
            Vec2::new(2.0, 10.0)
                .plus(&Vec2::new(5.0, 20.0));

        assert_eq!(result, Vec2::new(7.0, 30.0));
    }
}
