use pebble_rust_2026::GContext;

use crate::{animation::Status, grid::DigitBounds, types::Stroke};

pub struct Transition {
    strokes: &'static [Stroke],
}

impl Transition {
    pub const fn def(strokes: &'static [Stroke]) -> Self {
        Self { strokes }
    }

    pub fn render(&self, ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
        for stroke in self.strokes {
            stroke.render(ctx, bounds, progress);
        }

        if progress < self.duration() {
            Status::Active
        } else {
            Status::Complete
        }
    }

    pub(crate) fn duration(&self) -> i32 {
        self.strokes.iter().map(|e| e.duration()).max().unwrap_or(0)
    }
}
