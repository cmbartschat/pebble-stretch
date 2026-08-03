#![no_main]
#![no_std]

mod digits;
mod render;

extern crate alloc;

use core::{cell::RefCell, time::Duration};

use alloc::{boxed::Box, rc::Rc};

use pebble_rust_2026::{
    self as _, APP, Layer, Time, TimeUnits, Timer, Window, hex_color, resource_ids,
};

use crate::render::{TimeInterpolation, derive_layout, render_animated_time};

resource_ids!(resource_ids);

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut window = Window::new().unwrap();
    window.set_background_color(hex_color!("#000"));

    let mut layer = Layer::new(window.get_bounds().shrink(5)).unwrap();

    let now = Time::now();
    let now_seconds = now.epoch_seconds();
    let progress = Rc::new(RefCell::new(TimeInterpolation {
        from: Time::from_epoch_seconds(now_seconds + (11 * 60 * 60) + 11 * 60).to_local(),
        progress: 0,
        to: now.to_local(),
    }));

    layer.set_update_proc({
        let progress = progress.clone();
        Box::new(move |layer, mut ctx| {
            let progress = progress.borrow();
            let layout = derive_layout(layer.get_unobstructed_bounds());
            render_animated_time(&mut ctx, &layout, &progress);
        })
    });

    if true {
        APP.set_tick_handler(
            TimeUnits::Minute,
            Box::new({
                let mut layer = layer.clone();
                let progress = progress.clone();
                move || {
                    {
                        let mut progress = progress.borrow_mut();
                        *progress = TimeInterpolation {
                            progress: -progress.progress,
                            from: progress.to.clone(),
                            to: Time::now().to_local(),
                        };
                        layer.mark_dirty();
                    }
                    let progress = progress.clone();
                    let mut layer = layer.clone();
                    Timer::repeat(Duration::from_millis(25), move || {
                        layer.mark_dirty();
                        let mut progress = progress.borrow_mut();
                        progress.progress += 5;
                        progress.progress < 100
                    });
                }
            }),
        );
    } else {
        Timer::repeat(Duration::from_secs(3), {
            let mut layer = layer.clone();
            let progress = progress.clone();
            let mut seconds = 0;
            move || {
                seconds += 60;
                {
                    let mut progress = progress.borrow_mut();
                    *progress = TimeInterpolation {
                        progress: -progress.progress,
                        from: progress.to.clone(),
                        to: Time::from_epoch_seconds(seconds).to_local(),
                    };
                    layer.mark_dirty();
                }
                let progress = progress.clone();
                let mut layer = layer.clone();
                Timer::repeat(Duration::from_millis(25), move || {
                    layer.mark_dirty();
                    let mut progress = progress.borrow_mut();
                    progress.progress += 5;
                    progress.progress < 100
                });

                true
            }
        });
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
