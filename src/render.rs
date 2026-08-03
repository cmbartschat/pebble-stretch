use pebble_rust_2026::{GAlign, GContext, GPoint, GRect, LocalTime, color, resource_ids};

use crate::digits::{DigitBounds, render_digit};

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

fn get_digits(time: &LocalTime) -> (i32, i32, i32, i32) {
    let mut hour = time.hour();
    if hour == 0 {
        hour = 12;
    } else if hour > 12 {
        hour -= 12;
    }

    (
        hour.div_euclid(10),
        hour.rem_euclid(10),
        time.minute().div_euclid(10),
        time.minute().rem_euclid(10),
    )
}

pub struct TimeInterpolation {
    pub(crate) progress: i32,
    pub(crate) from: LocalTime,
    pub(crate) to: LocalTime,
}

pub fn render_interpolated_digit(
    ctx: &mut GContext,
    bounds: &DigitBounds,
    old: i32,
    new: i32,
    progress: i32,
) {
    if old == new {
        render_digit(new, ctx, bounds, 100);
    } else if progress < 0 {
        render_digit(old, ctx, bounds, -progress);
    } else {
        render_digit(new, ctx, bounds, progress);
    }
}

pub fn render_animated_time(ctx: &mut GContext, layout: &DigitLayout, inter: &TimeInterpolation) {
    ctx.set_stroke_width(layout.stroke_width);
    ctx.set_stroke_color(color::GCOLOR_WHITE);

    let mut digit = layout.first.clone();

    let from = get_digits(&inter.from);
    let to = get_digits(&inter.to);

    render_interpolated_digit(ctx, &digit, from.0, to.0, inter.progress);
    digit.base.y += layout.offset;
    render_interpolated_digit(ctx, &digit, from.1, to.1, inter.progress);
    digit.base.y += layout.offset;
    render_interpolated_digit(ctx, &digit, from.2, to.2, inter.progress);
    digit.base.y += layout.offset;
    render_interpolated_digit(ctx, &digit, from.3, to.3, inter.progress);
}
