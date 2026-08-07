use core::{cell::RefCell, time::Duration};

use alloc::rc::Rc;
use pebble_rust_2026::Timer;

use crate::animation::ContinuousInterpolation;

#[derive(Clone)]
pub struct DemoTrigger;

impl DemoTrigger {
    pub fn mount(inter: Rc<RefCell<ContinuousInterpolation>>) {
        let mut digits = (0, 0, 0, 5);
        let mut handler = move || {
            digits.0 = if digits.0 == 1 { 2 } else { 1 };
            digits.1 = (digits.1 + 1) % 6;
            digits.2 = (digits.2 + 1) % 10;
            digits.3 = (digits.3 + 1) % 10;
            ContinuousInterpolation::set_target_direct(&inter, digits);
            true
        };
        handler();
        Timer::repeat(Duration::from_millis(1500), handler);
    }
}
