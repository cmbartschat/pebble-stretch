use pebble_rust_2026::{GPoint, GSize};

#[derive(Clone)]
pub struct DigitBounds {
    pub base: GPoint,
    pub scale: i16,
}

impl DigitBounds {
    pub fn move_down(&self) -> GSize {
        GSize::new(0, self.scale * 2)
    }

    pub fn move_up(&self) -> GSize {
        GSize::new(0, -self.scale * 2)
    }
    pub fn move_half_down(&self) -> GSize {
        GSize::new(0, self.scale)
    }

    pub fn move_half_up(&self) -> GSize {
        GSize::new(0, -self.scale)
    }

    pub fn move_left(&self) -> GSize {
        GSize::new(-11 * self.scale, 0)
    }

    pub fn move_right(&self) -> GSize {
        GSize::new(11 * self.scale, 0)
    }

    pub fn left_top(&self) -> GPoint {
        GPoint::new(self.base.x, self.base.y)
    }
    pub fn left_cross(&self) -> GPoint {
        GPoint::new(self.base.x, self.base.y + self.scale)
    }
    pub fn left_bottom(&self) -> GPoint {
        GPoint::new(self.base.x, self.base.y + self.scale + self.scale)
    }
    pub fn right_top(&self) -> GPoint {
        GPoint::new(self.base.x + 11 * self.scale, self.base.y)
    }
    pub fn right_cross(&self) -> GPoint {
        GPoint::new(self.base.x + 11 * self.scale, self.base.y + self.scale)
    }
    pub fn right_bottom(&self) -> GPoint {
        GPoint::new(
            self.base.x + 11 * self.scale,
            self.base.y + self.scale + self.scale,
        )
    }

    pub fn move_left_one_segment(&self) -> GSize {
        GSize::new(-self.scale, 0)
    }
}
