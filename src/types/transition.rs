use pebble_rust_2026::GContext;

use crate::{grid::DigitBounds, types::Stroke};

pub struct Transition {
    strokes: &'static [Stroke],
}

impl Transition {
    pub const fn def(strokes: &'static [Stroke]) -> Self {
        Self { strokes }
    }

    pub fn render(&self, ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> bool {
        let mut pending = false;
        for stroke in self.strokes {
            pending = stroke.render(ctx, bounds, progress) || pending;
        }
        pending
    }
}
