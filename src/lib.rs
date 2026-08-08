#![no_main]
#![no_std]

mod animation;
mod config;
mod demo_trigger;
mod digits;
mod fast_time_trigger;
mod format;
mod grid;
mod render;
mod time;
mod time_trigger;
mod tricky_trigger;
mod types;

extern crate alloc;

use core::cell::RefCell;

use alloc::{boxed::Box, rc::Rc};

use pebble_rust_2026::{self as _, APP, Layer, Window, resource_ids};

use crate::{
    animation::{ContinuousInterpolation, InterpolatedTime, Status},
    config::load_config,
    demo_trigger::DemoTrigger,
    fast_time_trigger::FastTimeTrigger,
    render::{derive_layout, render_animated_time},
    time_trigger::TimeTrigger,
    tricky_trigger::TrickyTrigger,
};

resource_ids!(resource_ids);

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut window = Window::new().unwrap();

    let mut layer = Layer::new(window.get_bounds().shrink(5)).unwrap();

    let config = load_config({
        let mut window = window.retain();
        let mut layer = layer.clone();
        Box::new(move |config| {
            // Also fires on startup
            window.set_background_color(config.background_color);
            layer.mark_dirty();
        })
    });

    let progress = Rc::new(RefCell::new(InterpolatedTime::new()));

    layer.set_update_proc({
        let progress = progress.clone();
        Box::new(move |layer, mut ctx| {
            let mut progress = progress.borrow_mut();
            let layout = derive_layout(layer.get_unobstructed_bounds());
            let config = config.borrow();
            progress.stage_complete =
                render_animated_time(&mut ctx, &layout, &progress, &config) == Status::Complete;
        })
    });

    APP.unobstructed_area.subscribe(Box::new({
        let mut layer = layer.clone();
        move |_| {
            layer.mark_dirty();
        }
    }));

    window.add_child(&mut layer);

    layer.mark_dirty();

    let inter = ContinuousInterpolation::new(layer, progress).unwrap();

    #[allow(dead_code)]
    enum Trigger {
        Time,
        FastTime,
        Demo,
        Tricky,
    }

    match Trigger::Time {
        Trigger::Time => TimeTrigger::mount(inter),
        Trigger::FastTime => FastTimeTrigger::mount(inter),
        Trigger::Demo => DemoTrigger::mount(inter),
        Trigger::Tricky => TrickyTrigger::mount(inter),
    }

    APP.show(window);
    APP.event_loop();
    0
}
