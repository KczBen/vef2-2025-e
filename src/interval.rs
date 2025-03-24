// Yes there's a crate for this
// no I'm not using it

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        return Self {
            min,
            max,
        };
    }

    pub fn size(&self) -> f32 {
        return self.max - self.min;
    }

    pub fn contains(&self, value: f32) -> bool {
        return self.min <= value && value <= self.max;
    }

    pub fn surrounds(&self, value: f32) -> bool {
        return self.min < value && value < self.max;
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            return self.min;
        }
        
        if value > self.max {
            return self.max;
        }

        return value;
    }
}