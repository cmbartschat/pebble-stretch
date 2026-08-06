use pebble_rust_2026::GSize;

use crate::{grid::DigitBounds, time::TIME_SCALE};

#[derive(Copy, Clone, PartialEq)]
pub enum Dir {
    R,
    L,
    U,
    D,
    Hu,
    Hd,
    OneL,
}

impl Dir {
    pub fn project(self, bounds: &DigitBounds) -> GSize {
        match self {
            Self::R => bounds.move_right(),
            Self::L => bounds.move_left(),
            Self::U => bounds.move_up(),
            Self::D => bounds.move_down(),
            Self::Hu => bounds.move_half_up(),
            Self::Hd => bounds.move_half_down(),
            Self::OneL => bounds.move_left_one_segment(),
        }
    }

    pub fn duration(&self) -> i32 {
        TIME_SCALE
            * match self {
                Self::R => 8,
                Self::L => 8,
                Self::U => 2,
                Self::D => 2,
                Self::Hu => 1,
                Self::Hd => 1,
                Self::OneL => 1,
            }
    }
}
