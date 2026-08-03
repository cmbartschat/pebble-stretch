use pebble_rust_2026::{GAlign, GContext, GPoint, GRect, Time, color, log_fmt, resource_ids};

use crate::digits::{DigitBounds, render_digit};

resource_ids!(resource_ids);

pub fn render_time(ctx: &mut GContext, bounds: GRect) {
    let available = bounds.size.w.min(bounds.size.h);
    let base_scale = available.div_euclid(50);

    const STROKE_COUNT: i16 = 12;
    const SPACE_COUNT: i16 = 11;
    let space_width: i16 = base_scale * 2;
    let stroke_width = (available - (space_width * STROKE_COUNT)) / SPACE_COUNT;

    unsafe {
        log_fmt!(
            c"stroke_width: %i, space_width: %i, base_scale: %i",
            stroke_width as i32,
            space_width as i32,
            base_scale as i32
        );
    }

    ctx.set_stroke_width(stroke_width as u8);
    ctx.set_stroke_color(color::GCOLOR_WHITE);

    // let total_height = (stroke_width as i16) * STROKE_COUNT + space_width * SPACE_COUNT;

    let scale: i16 = space_width + stroke_width;
    let visible_bounds = GRect::new(0, 0, 11 * scale, 11 * scale).align(&bounds, GAlign::Center);

    let base_y = visible_bounds.origin.y;
    let base_x = visible_bounds.origin.x;

    let time = Time::now().to_local();
    let hour = (time.hour() - 1) % 12 + 1;
    let digit_0 = hour.div_euclid(10);
    let digit_1 = hour.rem_euclid(10);
    let digit_2 = time.minute().div_euclid(10);
    let digit_3 = time.minute().rem_euclid(10);

    render_digit(
        digit_0,
        ctx,
        &DigitBounds {
            base: GPoint::new(base_x, base_y),
            scale,
        },
    );

    render_digit(
        digit_1,
        ctx,
        &DigitBounds {
            base: GPoint::new(base_x, base_y + 3 * scale),
            scale,
        },
    );

    render_digit(
        digit_2,
        ctx,
        &DigitBounds {
            base: GPoint::new(base_x, base_y + 6 * scale),
            scale,
        },
    );

    render_digit(
        digit_3,
        ctx,
        &DigitBounds {
            base: GPoint::new(base_x, base_y + 9 * scale),
            scale,
        },
    );
}
