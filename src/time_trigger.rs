use core::{cell::RefCell, time::Duration};

use alloc::{boxed::Box, rc::Rc};
use pebble_rust_2026::{APP, Time, TimeUnits, Timer};

use crate::{animation::ContinuousInterpolation, format::get_digits};

#[derive(Clone)]
pub struct TimeTrigger;

impl TimeTrigger {
    pub fn mount(inter: Rc<RefCell<ContinuousInterpolation>>) {
        Timer::once(Duration::from_millis(500), {
            let inter = inter.clone();
            move || {
                ContinuousInterpolation::set_target_direct(
                    &inter,
                    get_digits(&Time::now().to_local()),
                );
            }
        });

        let mut first = true;
        APP.set_tick_handler(
            TimeUnits::Minute,
            Box::new({
                move || {
                    let digits = get_digits(&Time::now().to_local());
                    if first {
                        first = false;
                    } else {
                        ContinuousInterpolation::set_target(&inter, digits);
                    }
                }
            }),
        );
    }
}
