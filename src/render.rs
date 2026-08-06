use pebble_rust_2026::{GAlign, GContext, GPoint, GRect, GSize, color, resource_ids};

use crate::{
    animation::{InterpolatedDigit, InterpolatedTime},
    digits::*,
    time::MAX_TIME,
};

resource_ids!(resource_ids);

#[derive(Clone)]
pub struct DigitLayout {
    first: DigitBounds,
    stroke_width: u8,
    offset: i16,
}

pub fn derive_layout(bounds: GRect) -> DigitLayout {
    let available = bounds.size.w.min(bounds.size.h);
    let base_scale = available.div_euclid(50);

    const STROKE_COUNT: i16 = 12;
    const SPACE_COUNT: i16 = 11;
    let stroke_width = base_scale * 4;
    let space_width: i16 = (available - (stroke_width * STROKE_COUNT)) / SPACE_COUNT;

    let scale: i16 = space_width + stroke_width;
    let visible_bounds = GRect::new(0, 0, 11 * scale, 11 * scale).align(&bounds, GAlign::Center);

    let base_y = visible_bounds.origin.y;
    let base_x = visible_bounds.origin.x;

    DigitLayout {
        first: DigitBounds {
            base: GPoint::new(base_x, base_y),
            scale,
        },
        offset: 3 * scale,
        stroke_width: stroke_width as u8,
    }
}

pub fn render_interpolated_digit(
    ctx: &mut GContext,
    bounds: &DigitBounds,
    digit: &InterpolatedDigit,
    progress: i32,
) {
    match digit {
        InterpolatedDigit::Stable(e) => render_digit(*e, ctx, bounds, MAX_TIME),
        InterpolatedDigit::Change(old, new) if *old == 0 && *new == 1 => {
            render_0_1(ctx, bounds, progress)
        }
        InterpolatedDigit::Change(old, new) if *old == 1 && *new == 2 => {
            render_1_2(ctx, bounds, progress)
        }
        InterpolatedDigit::Change(old, new) if *old == 2 && *new == 3 => {
            render_2_3(ctx, bounds, progress)
        }
        InterpolatedDigit::Change(old, new) if *old == 3 && *new == 4 => {
            render_3_4(ctx, bounds, progress)
        }
        // InterpolatedDigit::Change(old, new) if *old == 4 && *new == 5 => {
        //     render_4_5(ctx, bounds, progress)
        // }
        // InterpolatedDigit::Change(old, new) if *old == 5 && *new == 6 => {
        //     render_5_6(ctx, bounds, progress)
        // }
        // InterpolatedDigit::Change(old, new) if *old == 6 && *new == 7 => {
        //     render_6_7(ctx, bounds, progress)
        // }
        // InterpolatedDigit::Change(old, new) if *old == 7 && *new == 8 => {
        //     render_7_8(ctx, bounds, progress)
        // }
        // InterpolatedDigit::Change(old, new) if *old == 8 && *new == 9 => {
        //     render_8_9(ctx, bounds, progress)
        // }
        // InterpolatedDigit::Change(old, new) if *old == 9 && *new == 0 => {
        //     render_9_0(ctx, bounds, progress)
        // }
        // InterpolatedDigit::Change(old, new) if *old == 2 && *new == 1 => {
        //     render_2_1(ctx, bounds, progress)
        // }
        InterpolatedDigit::Change(old, new) if *old == 1 && *new == 0 => {
            render_1_0(ctx, bounds, progress)
        }
        // InterpolatedDigit::Change(old, new) if *old == 5 && *new == 0 => {
        //     render_5_0(ctx, bounds, progress)
        // }
        InterpolatedDigit::Change(_, new) => render_digit(*new, ctx, bounds, progress),
    };
}

pub fn render_animated_time(ctx: &mut GContext, layout: &DigitLayout, inter: &InterpolatedTime) {
    ctx.set_stroke_width(layout.stroke_width);
    ctx.set_stroke_color(color::GCOLOR_WHITE);

    let mut digit = layout.first.clone();
    render_interpolated_digit(ctx, &digit, &inter.digits.0, inter.progress);
    digit.base.y += layout.offset;
    render_interpolated_digit(ctx, &digit, &inter.digits.1, inter.progress);
    digit.base.y += layout.offset;
    render_interpolated_digit(ctx, &digit, &inter.digits.2, inter.progress);
    digit.base.y += layout.offset;
    render_interpolated_digit(ctx, &digit, &inter.digits.3, inter.progress);
}

pub fn render_line(ctx: &mut GContext, from: GPoint, to: GSize, budget: &mut i32, allowance: i32) {
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
