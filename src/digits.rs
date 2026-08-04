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

pub fn render_0_1(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    bottom(ctx, bounds, &mut scratch, 1);

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_up(),
            &mut progress,
            18,
        );
        line(
            ctx,
            bounds.left_top(),
            bounds.move_right(),
            &mut progress,
            64,
        );
        line(
            ctx,
            bounds.right_top(),
            bounds.move_down(),
            &mut progress,
            18,
        );
        return;
    }

    line(
        ctx,
        GPoint::new(
            bounds.base.x + bounds.scale * 6,
            bounds.base.y + bounds.scale * 2,
        ),
        GSize::new(0, -bounds.scale * 2),
        &mut progress,
        55,
    );

    line(
        ctx,
        GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
        GSize::new(-bounds.scale, 0),
        &mut progress,
        45,
    );
}

pub fn render_1_2(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    bottom(ctx, bounds, &mut scratch, 1);

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            GPoint::new(
                bounds.base.x + bounds.scale * 6,
                bounds.base.y + bounds.scale * 2,
            ),
            GSize::new(0, -bounds.scale * 2),
            &mut progress,
            55,
        );

        line(
            ctx,
            GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
            GSize::new(-bounds.scale, 0),
            &mut progress,
            45,
        );
    } else {
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_half_up(),
            &mut progress,
            10,
        );
        line(
            ctx,
            bounds.left_cross(),
            bounds.move_right(),
            &mut progress,
            40,
        );
        line(
            ctx,
            bounds.right_cross(),
            bounds.move_half_up(),
            &mut progress,
            10,
        );
        line(
            ctx,
            bounds.right_top(),
            bounds.move_left(),
            &mut progress,
            40,
        );
    }
}

pub fn render_2_3(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    top(ctx, bounds, &mut scratch, 1);
    cross(ctx, bounds, &mut scratch, 1);
    bottom(ctx, bounds, &mut scratch, 1);
    line(
        ctx,
        bounds.right_top(),
        bounds.move_half_down(),
        &mut scratch,
        1,
    );

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_half_up(),
            &mut progress,
            100,
        );
    } else {
        line(
            ctx,
            bounds.right_cross(),
            bounds.move_half_down(),
            &mut progress,
            100,
        );
    }
}

pub fn render_3_4(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    cross(ctx, bounds, &mut scratch, 1);
    line(ctx, bounds.right_top(), bounds.move_down(), &mut scratch, 1);

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.right_top(),
            bounds.move_left(),
            &mut progress,
            50,
        );
        line(
            ctx,
            bounds.right_bottom(),
            bounds.move_left(),
            &mut progress,
            50,
        );
    } else {
        line(
            ctx,
            bounds.left_cross(),
            bounds.move_half_up(),
            &mut progress,
            50,
        );
        line(
            ctx,
            bounds.right_cross(),
            bounds.move_half_down(),
            &mut progress,
            50,
        );
    }
}

pub fn render_4_5(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    cross(ctx, bounds, &mut scratch, 1);
    line(
        ctx,
        bounds.right_cross(),
        bounds.move_half_down(),
        &mut scratch,
        1,
    );
    line(
        ctx,
        bounds.left_cross(),
        bounds.move_half_up(),
        &mut scratch,
        1,
    );

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.right_cross(),
            bounds.move_half_up(),
            &mut progress,
            100,
        );
    } else {
        line(
            ctx,
            bounds.left_top(),
            bounds.move_right(),
            &mut progress,
            50,
        );
        line(
            ctx,
            bounds.right_bottom(),
            bounds.move_left(),
            &mut progress,
            50,
        );
    }
}

pub fn render_5_6(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    render_5(ctx, bounds, 100);
    if progress > 0 {
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_half_up(),
            &mut progress,
            100,
        );
    }
}

pub fn render_6_7(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    line(ctx, bounds.left_top(), bounds.move_right(), &mut scratch, 1);

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.left_top(),
            bounds.move_down(),
            &mut progress,
            11,
        );
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_right(),
            &mut progress,
            42,
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
            42,
        );
    } else {
        line(
            ctx,
            bounds.right_top(),
            bounds.move_down(),
            &mut progress,
            100,
        );
    }
}

pub fn render_7_8(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    render_7(ctx, bounds, 100);
    if progress > 0 {
        line(
            ctx,
            bounds.left_top(),
            bounds.move_down(),
            &mut progress,
            12,
        );
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_right(),
            &mut progress,
            44,
        );
        line(
            ctx,
            bounds.right_cross(),
            bounds.move_left(),
            &mut progress,
            44,
        );
    }
}

pub fn render_8_9(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    render_9(ctx, bounds, 100);
    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.right_bottom(),
            bounds.move_left(),
            &mut progress,
            85,
        );
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_half_up(),
            &mut progress,
            15,
        );
    }
}

pub fn render_9_0(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
    let mut scratch = 100;
    line(ctx, bounds.left_top(), bounds.move_right(), &mut scratch, 1);
    line(
        ctx,
        bounds.left_top(),
        bounds.move_half_down(),
        &mut scratch,
        1,
    );
    line(ctx, bounds.right_top(), bounds.move_down(), &mut scratch, 1);

    if progress < 0 {
        progress *= -1;
        line(
            ctx,
            bounds.left_cross(),
            bounds.move_right(),
            &mut progress,
            100,
        );
    } else {
        line(
            ctx,
            bounds.right_bottom(),
            bounds.move_left(),
            &mut progress,
            89,
        );
        line(
            ctx,
            bounds.left_bottom(),
            bounds.move_half_up(),
            &mut progress,
            11,
        );
    }
}

pub fn render_1_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    render_0_1(ctx, bounds, -progress);
}

pub fn render_5_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    if progress < 0 {
        render_5(ctx, bounds, -progress);
    } else {
        render_0(ctx, bounds, progress);
    }
}

pub fn render_2_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    render_1_2(ctx, bounds, -progress);
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
