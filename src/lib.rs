#![no_main]
#![no_std]

mod digits;
mod render;

extern crate alloc;

use alloc::boxed::Box;

use pebble_rust_2026::{self as _, APP, Layer, TimeUnits, Window, hex_color, resource_ids};

use crate::render::render_time;

resource_ids!(resource_ids);

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut window = Window::new().unwrap();
    window.set_background_color(hex_color!("#000"));

    let mut layer = Layer::new(window.get_bounds().shrink(5)).unwrap();

    layer.set_update_proc(Box::new(|layer, mut ctx| {
        render_time(&mut ctx, layer.get_unobstructed_bounds());
    }));

    APP.set_tick_handler(
        TimeUnits::Minute,
        Box::new({
            let mut layer = layer.clone();
            move || {
                layer.mark_dirty();
            }
        }),
    );

    APP.unobstructed_area.subscribe(Box::new({
        let mut layer = layer.clone();
        move |_| {
            layer.mark_dirty();
        }
    }));

    window.add_child(&mut layer);

    APP.show(window);
    APP.event_loop();
    0
}
