use crate::kurbo::Size;

#[derive(Debug, Default, Clone, Copy)]
pub struct Constraints {
    pub min: Size,
    pub max: Size,
}

impl Constraints {
    pub fn with_min(&self, min: Size) -> Self {
        Constraints { min, max: self.max }
    }

    pub fn with_max(&self, max: Size) -> Self {
        Constraints { min: self.min, max }
    }

    pub fn with_max_height(&self, height: f64) -> Self {
        Constraints {
            min: self.min,
            max: Size {
                width: self.max.width,
                height,
            },
        }
    }
}
