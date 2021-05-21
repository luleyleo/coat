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

    pub fn shrink(&self, size: Size) -> Self {
        Self::clipped(&Constraints {
            min: self.min - size,
            max: self.max - size,
        })
    }

    pub fn clipped(&self) -> Self {
        let max = Size {
            width: self.max.width,
            height: self.max.height,
        };
        let min = Size {
            width: self.min.width.max(0.0).min(max.width),
            height: self.min.height.max(0.0).min(max.height),
        };
        Constraints { min, max }
    }
}
