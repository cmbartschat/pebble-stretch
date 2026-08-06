use pebble_rust_2026::GPoint;

use crate::grid::DigitBounds;

#[derive(Copy, Clone, PartialEq)]
pub enum Pos {
    Tl,
    Tr,
    TOne,
    BOne,
    Cl,
    Cr,
    Bl,
    Br,
}

impl Pos {
    pub fn project(self, grid: &DigitBounds) -> GPoint {
        match self {
            Self::Tl => grid.left_top(),
            Self::Tr => grid.right_top(),
            Self::TOne => GPoint::new(grid.base.x + grid.scale * 6, grid.base.y),
            Self::BOne => GPoint::new(grid.base.x + grid.scale * 6, grid.base.y + grid.scale * 2),
            Self::Cl => grid.left_cross(),
            Self::Cr => grid.right_cross(),
            Self::Bl => grid.left_bottom(),
            Self::Br => grid.right_bottom(),
        }
    }
}
