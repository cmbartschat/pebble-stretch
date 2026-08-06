#![no_main]
#![no_std]

mod animation;
mod digits;
mod format;
mod grid;
mod render;
mod time;
mod types;

extern crate alloc;

use core::{cell::RefCell, time::Duration};

use alloc::{boxed::Box, rc::Rc};

use pebble_rust_2026::{
    self as _, APP, Layer, Time, TimeUnits, Timer, Window, hex_color, resource_ids,
};

use crate::{
    animation::InterpolatedTime,
    render::{derive_layout, render_animated_time},
    time::{FRAME_TIME, MAX_TIME, TIME_STEP},
};

resource_ids!(resource_ids);

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut window = Window::new().unwrap();
    window.set_background_color(hex_color!("#000"));

    let mut layer = Layer::new(window.get_bounds().shrink(5)).unwrap();

    let progress = Rc::new(RefCell::new(InterpolatedTime::new(Time::now())));

    layer.set_update_proc({
        let progress = progress.clone();
        Box::new(move |layer, mut ctx| {
            let progress = progress.borrow();
            let layout = derive_layout(layer.get_unobstructed_bounds());
            render_animated_time(&mut ctx, &layout, &progress);
        })
    });

    if false {
        APP.set_tick_handler(
            TimeUnits::Minute,
            Box::new({
                let mut layer = layer.clone();
                let progress = progress.clone();
                move || {
                    {
                        progress.borrow_mut().animate_to_time(Time::now());
                        layer.mark_dirty();
                    }
                    let progress = progress.clone();
                    let mut layer = layer.clone();
                    Timer::repeat(Duration::from_millis(FRAME_TIME), move || {
                        layer.mark_dirty();
                        progress.borrow_mut().advance(2)
                    });
                }
            }),
        );
    } else {
        let mut layer = layer.clone();
        let progress = progress.clone();
        let digits = (0, 0, 0, 5);

        {
            let mut progress = progress.borrow_mut();
            progress.animate_to_digits(digits);
            progress.advance(MAX_TIME);
        }

        type Type = Option<Box<dyn FnMut() + 'static>>;
        let start_next_animation: Rc<RefCell<Type>> = Rc::new(RefCell::new(None));

        let start_next_animation_inner = {
            let progress = progress.clone();
            let start_next_animation = start_next_animation.clone();
            let mut digits = digits;
            move || {
                digits.0 = if digits.0 == 1 { 2 } else { 1 };
                digits.1 = (digits.1 + 1) % 6;
                digits.2 = (digits.2 + 1) % 10;
                digits.3 = (digits.3 + 1) % 10;
                {
                    progress.borrow_mut().animate_to_digits(digits);
                    layer.mark_dirty();
                }

                let start_next_animation = start_next_animation.clone();
                let mut layer = layer.clone();
                let progress = progress.clone();
                Timer::repeat(Duration::from_millis(FRAME_TIME), move || {
                    layer.mark_dirty();
                    if progress.borrow_mut().advance(TIME_STEP) {
                        true
                    } else {
                        let start_next_animation = start_next_animation.clone();
                        Timer::once(Duration::from_millis(1000), move || {
                            (start_next_animation.borrow_mut().as_mut().unwrap())();
                        });
                        false
                    }
                });
            }
        };
        start_next_animation
            .borrow_mut()
            .replace(Box::new(start_next_animation_inner));

        (start_next_animation.borrow_mut().as_mut().unwrap())();
    }

    APP.unobstructed_area.subscribe(Box::new({
        let mut layer = layer.clone();
        move |_| {
            layer.mark_dirty();
        }
    }));

    layer.mark_dirty();
    window.add_child(&mut layer);

    APP.show(window);
    APP.event_loop();
    0
}
