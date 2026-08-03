use pebble_rust_2026::{GContext, GPoint, GSize};

#[derive(Clone)]
pub struct DigitBounds {
    pub base: GPoint,
    pub scale: i16,
}

fn line(ctx: &mut GContext, from: GPoint, to: GSize, budget: &mut i32, allowance: i32) {
    if *budget <= 0 {
        return;
    }

    let spent = allowance.min(*budget);
    *budget -= allowance;

    let offset_x = to.w as i32 * spent / allowance;
    let offset_y = to.h as i32 * spent / allowance;

    ctx.draw_line(
        from,
        GPoint::new(from.x + offset_x as i16, from.y + offset_y as i16),
    );
}

fn left(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x, bounds.base.y),
        GSize::new(0, 2 * bounds.scale),
        budget,
        allowance,
    );
}

fn right(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x + bounds.scale * 11, bounds.base.y),
        GSize::new(0, 2 * bounds.scale),
        budget,
        allowance,
    );
}

fn left_top(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x, bounds.base.y + bounds.scale),
        GSize::new(0, -bounds.scale),
        budget,
        allowance,
    );
}

fn left_bottom(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x, bounds.base.y + bounds.scale),
        GSize::new(0, bounds.scale),
        budget,
        allowance,
    );
}

fn right_top(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale,
        ),
        GSize::new(0, -bounds.scale),
        budget,
        allowance,
    );
}

fn right_bottom(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(
            bounds.base.x + bounds.scale * 11,
            bounds.base.y + bounds.scale,
        ),
        GSize::new(0, bounds.scale),
        budget,
        allowance,
    );
}

fn top(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x, bounds.base.y),
        GSize::new(11 * bounds.scale, 0),
        budget,
        allowance,
    );
}

fn cross(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(
            bounds.base.x + 11 * bounds.scale,
            bounds.base.y + bounds.scale,
        ),
        GSize::new(-11 * bounds.scale, 0),
        budget,
        allowance,
    );
}

fn bottom(ctx: &mut GContext, bounds: &DigitBounds, budget: &mut i32, allowance: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x, bounds.base.y + 2 * bounds.scale),
        GSize::new(11 * bounds.scale, 0),
        budget,
        allowance,
    );
}

fn render_0(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 40);
    bottom(ctx, bounds, &mut progress, 40);
    left(ctx, bounds, &mut progress, 10);
    right(ctx, bounds, &mut progress, 10);
}

fn render_1(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        GPoint::new(bounds.base.x + bounds.scale * 5, bounds.base.y),
        GSize::new(bounds.scale, 0),
        &mut progress,
        20,
    );

    line(
        ctx,
        GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
        GSize::new(0, bounds.scale * 2),
        &mut progress,
        20,
    );

    bottom(ctx, bounds, &mut progress, 60);
}

fn render_2(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 30);
    cross(ctx, bounds, &mut progress, 30);
    bottom(ctx, bounds, &mut progress, 30);
    left_bottom(ctx, bounds, &mut progress, 5);
    right_top(ctx, bounds, &mut progress, 5);
}

fn render_3(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 30);
    cross(ctx, bounds, &mut progress, 30);
    bottom(ctx, bounds, &mut progress, 30);
    right(ctx, bounds, &mut progress, 8);
}

fn render_4(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    cross(ctx, bounds, &mut progress, 73);
    left_top(ctx, bounds, &mut progress, 9);
    right(ctx, bounds, &mut progress, 18);
}

fn render_5(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 30);
    cross(ctx, bounds, &mut progress, 30);
    bottom(ctx, bounds, &mut progress, 30);
    left_top(ctx, bounds, &mut progress, 5);
    right_bottom(ctx, bounds, &mut progress, 5);
}

fn render_6(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 29);
    cross(ctx, bounds, &mut progress, 29);
    bottom(ctx, bounds, &mut progress, 29);
    left(ctx, bounds, &mut progress, 8);
    right_bottom(ctx, bounds, &mut progress, 5);
}

fn render_7(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 80);
    right(ctx, bounds, &mut progress, 20);
}

fn render_8(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 28);
    cross(ctx, bounds, &mut progress, 28);
    bottom(ctx, bounds, &mut progress, 28);
    left(ctx, bounds, &mut progress, 8);
    right(ctx, bounds, &mut progress, 8);
}

fn render_9(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    top(ctx, bounds, &mut progress, 42);
    cross(ctx, bounds, &mut progress, 42);
    left_top(ctx, bounds, &mut progress, 6);
    right(ctx, bounds, &mut progress, 10);
}

pub fn render_digit(d: i32, ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
    })(ctx, bounds, progress)
}
