use pebble_rust_2026::{GContext, GPoint};

pub struct DigitBounds {
    pub base: GPoint,
    pub scale: i16,
    // pub amount: u8,
}

fn left(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x, bounds.base.y),
        GPoint::new(bounds.base.x, bounds.base.y + 2 * bounds.scale),
    );
}

fn right(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x + bounds.scale * 11, bounds.base.y),
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + 2 * bounds.scale,
        ),
    );
}

fn top(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x, bounds.base.y),
        GPoint::new(bounds.base.x + bounds.scale * 11, bounds.base.y),
    );
}

fn left_top(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x, bounds.base.y),
        GPoint::new(bounds.base.x, bounds.base.y + bounds.scale),
    );
}

fn left_bottom(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x, bounds.base.y + bounds.scale),
        GPoint::new(bounds.base.x, bounds.base.y + 2 * bounds.scale),
    );
}

fn right_top(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x + bounds.scale * 11, bounds.base.y),
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale,
        ),
    );
}

fn right_bottom(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale,
        ),
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale * 2,
        ),
    );
}

fn cross(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x, bounds.base.y + bounds.scale),
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale,
        ),
    );
}

fn bottom(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x, bounds.base.y + bounds.scale * 2),
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale * 2,
        ),
    );
}

fn render_0(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    right(ctx, bounds);
    left(ctx, bounds);
    bottom(ctx, bounds);
}

fn render_1(ctx: &mut GContext, bounds: &DigitBounds) {
    ctx.draw_line(
        GPoint::new(bounds.base.x + bounds.scale * 5, bounds.base.y),
        GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
    );

    ctx.draw_line(
        GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
        GPoint::new(
            bounds.base.x + bounds.scale * 6,
            bounds.base.y + bounds.scale * 2,
        ),
    );

    bottom(ctx, bounds);
}

fn render_2(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    cross(ctx, bounds);
    bottom(ctx, bounds);
    left_bottom(ctx, bounds);
    right_top(ctx, bounds);
}

fn render_3(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    cross(ctx, bounds);
    bottom(ctx, bounds);
    right(ctx, bounds);
}

fn render_4(ctx: &mut GContext, bounds: &DigitBounds) {
    left_top(ctx, bounds);
    cross(ctx, bounds);
    right(ctx, bounds);
}

fn render_5(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    cross(ctx, bounds);
    bottom(ctx, bounds);
    left_top(ctx, bounds);
    right_bottom(ctx, bounds);
}

fn render_6(ctx: &mut GContext, bounds: &DigitBounds) {
    left(ctx, bounds);
    top(ctx, bounds);
    cross(ctx, bounds);
    bottom(ctx, bounds);
    right_bottom(ctx, bounds);
}

fn render_7(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    right(ctx, bounds);
}

fn render_8(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    right(ctx, bounds);
    cross(ctx, bounds);
    left(ctx, bounds);
    bottom(ctx, bounds);
}

fn render_9(ctx: &mut GContext, bounds: &DigitBounds) {
    top(ctx, bounds);
    right(ctx, bounds);
    cross(ctx, bounds);
    left_top(ctx, bounds);
}

pub fn render_digit(d: i32, ctx: &mut GContext, bounds: &DigitBounds) {
    (match d {
        0 => render_0,
        1 => render_1,
        2 => render_2,
        3 => render_3,
        4 => render_4,
        5 => render_5,
        6 => render_6,
        7 => render_7,
        8 => render_8,
        9 => render_9,
        _ => return,
    })(ctx, bounds)
}
