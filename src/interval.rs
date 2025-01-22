// Yes there's a crate for this
// no I'm not using it

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        return Self {
            min,
            max,
        };
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, value: f64) -> bool {
        return self.min <= value && value <= self.max;
    }

    pub fn surrounds(&self, value: f64) -> bool {
        return self.min < value && value < self.max;
    }
}