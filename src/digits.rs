use pebble_rust_2026::{GContext, GPoint, GSize};

pub use crate::grid::DigitBounds;

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
    line(
        ctx,
        bounds.left_top(),
        bounds.move_right(),
        &mut progress,
        40,
    );
    line(
        ctx,
        bounds.right_top(),
        bounds.move_down(),
        &mut progress,
        10,
    );
    line(
        ctx,
        bounds.right_bottom(),
        bounds.move_left(),
        &mut progress,
        40,
    );
    line(
        ctx,
        bounds.left_bottom(),
        bounds.move_up(),
        &mut progress,
        10,
    );
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
    line(
        ctx,
        bounds.left_top(),
        bounds.move_right(),
        &mut progress,
        30,
    );
    line(
        ctx,
        bounds.right_top(),
        bounds.move_half_down(),
        &mut progress,
        5,
    );
    line(
        ctx,
        bounds.right_cross(),
        bounds.move_left(),
        &mut progress,
        30,
    );
    line(
        ctx,
        bounds.left_cross(),
        bounds.move_half_down(),
        &mut progress,
        5,
    );
    line(
        ctx,
        bounds.left_bottom(),
        bounds.move_right(),
        &mut progress,
        30,
    );
}

fn render_3(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        bounds.left_top(),
        bounds.move_right(),
        &mut progress,
        30,
    );
    line(
        ctx,
        bounds.right_top(),
        bounds.move_down(),
        &mut progress,
        10,
    );
    line(
        ctx,
        bounds.right_bottom(),
        bounds.move_left(),
        &mut progress,
        30,
    );
    line(
        ctx,
        bounds.left_cross(),
        bounds.move_right(),
        &mut progress,
        30,
    );
}

fn render_4(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        bounds.left_top(),
        bounds.move_half_down(),
        &mut progress,
        9,
    );
    line(
        ctx,
        bounds.left_cross(),
        bounds.move_right(),
        &mut progress,
        73,
    );
    line(
        ctx,
        bounds.right_top(),
        bounds.move_down(),
        &mut progress,
        18,
    );
}

fn render_5(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        bounds.right_top(),
        bounds.move_left(),
        &mut progress,
        30,
    );
    line(
        ctx,
        bounds.left_top(),
        bounds.move_half_down(),
        &mut progress,
        5,
    );
    line(
        ctx,
        bounds.left_cross(),
        bounds.move_right(),
        &mut progress,
        30,
    );
    line(
        ctx,
        bounds.right_cross(),
        bounds.move_half_down(),
        &mut progress,
        5,
    );
    line(
        ctx,
        bounds.right_bottom(),
        bounds.move_left(),
        &mut progress,
        30,
    );
}

fn render_6(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        bounds.right_top(),
        bounds.move_left(),
        &mut progress,
        29,
    );

    line(ctx, bounds.left_top(), bounds.move_down(), &mut progress, 8);

    line(
        ctx,
        bounds.left_bottom(),
        bounds.move_right(),
        &mut progress,
        29,
    );

    line(
        ctx,
        bounds.right_bottom(),
        bounds.move_half_up(),
        &mut progress,
        5,
    );

    line(
        ctx,
        bounds.right_cross(),
        bounds.move_left(),
        &mut progress,
        29,
    );
}

fn render_7(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        bounds.right_bottom(),
        bounds.move_up(),
        &mut progress,
        20,
    );
    line(
        ctx,
        bounds.right_top(),
        bounds.move_left(),
        &mut progress,
        80,
    );
}

fn render_8(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    line(
        ctx,
        bounds.right_top(),
        bounds.move_left(),
        &mut progress,
        28,
    );

    line(ctx, bounds.left_top(), bounds.move_down(), &mut progress, 8);

    line(
        ctx,
        bounds.left_bottom(),
        bounds.move_right(),
        &mut progress,
        28,
    );

    line(
        ctx,
        bounds.right_bottom(),
        bounds.move_up(),
        &mut progress,
        8,
    );

    line(
        ctx,
        bounds.right_cross(),
        bounds.move_left(),
        &mut progress,
        28,
    );
}

fn render_9(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    cross(ctx, bounds, &mut progress, 42);
    line(
        ctx,
        bounds.left_cross(),
        bounds.move_half_up(),
        &mut progress,
        6,
    );
    top(ctx, bounds, &mut progress, 42);
    line(
        ctx,
        bounds.right_top(),
        bounds.move_down(),
        &mut progress,
        6,
    );
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
