use pebble_rust_2026::GContext;

use crate::{grid::DigitBounds, types::seg::Seg};

pub struct Stroke {
    back: bool,
    delay: i32,
    segments: &'static [Seg],
}

impl Stroke {
    pub const fn forward(delay: i32, segments: &'static [Seg]) -> Self {
        Self {
            back: false,
            delay,
            segments,
        }
    }

    pub const fn back(delay: i32, segments: &'static [Seg]) -> Self {
        Self {
            back: true,
            delay,
            segments,
        }
    }

    pub fn duration(&self) -> i32 {
        self.delay + self.segments.iter().map(|e| e.duration()).sum::<i32>()
    }

    pub fn render(&self, ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) -> bool {
        progress -= self.delay;
        if self.back {
            progress = self.duration() - progress;
        }
        for seg in self.segments {
            seg.render(ctx, bounds, &mut progress);
        }
        progress <= 0
    }
}
