use core::{cell::RefCell, time::Duration};

use alloc::rc::Rc;
use pebble_rust_2026::{Time, Timer};

use crate::{animation::ContinuousInterpolation, format::get_digits};

#[derive(Clone)]
pub struct FastTimeTrigger;

impl FastTimeTrigger {
    pub fn mount(inter: Rc<RefCell<ContinuousInterpolation>>) {
        let mut passed = 0;
        let mut handler = move || {
            passed += 60;
            ContinuousInterpolation::set_target(
                &inter,
                get_digits(&Time::from_epoch_seconds(passed).to_utc()),
            );
            true
        };
        handler();
        Timer::repeat(Duration::from_secs(3), handler);
    }
}
