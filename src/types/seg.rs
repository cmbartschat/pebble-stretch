use crate::{
    render::render_line,
    types::{Dir, Pos},
};

#[derive(Copy, Clone, PartialEq)]
pub struct Seg {
    pos: Pos,
    dir: Dir,
}

impl Seg {
    pub const fn def(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }

    pub fn duration(&self) -> i32 {
        self.dir.duration()
    }

    pub fn render(
        &self,
        ctx: &mut pebble_rust_2026::GContext,
        bounds: &crate::grid::DigitBounds,
        progress: &mut i32,
    ) {
        render_line(
            ctx,
            self.pos.project(bounds),
            self.dir.project(bounds),
            progress,
            self.duration(),
        );
    }
}
