use core::{cell::RefCell, time::Duration};

use alloc::rc::Rc;
use pebble_rust_2026::Timer;

use crate::animation::ContinuousInterpolation;

#[derive(Clone)]
pub struct TrickyTrigger;

static CASES: [((i32, i32, i32, i32), bool); 6] = [
    ((1, 1, 5, 9), true),
    ((1, 2, 0, 0), false),
    ((0, 9, 5, 9), true),
    ((1, 0, 0, 0), false),
    ((1, 0, 0, 1), false),
    ((1, 0, 0, 9), false),
];

impl TrickyTrigger {
    pub fn mount(inter: Rc<RefCell<ContinuousInterpolation>>) {
        ContinuousInterpolation::set_target_direct(&inter, CASES[0].0);

        let mut index: usize = 0;
        let mut stable_count = 0;

        let mut update = move || {
            {
                if !inter.borrow_mut().finished() {
                    stable_count = 0;
                    return;
                }
            }
            stable_count += 1;
            if stable_count < 5 {
                return;
            }
            stable_count = 0;
            index = (index + 1) % CASES.len();
            let case = &CASES[index];
            if case.1 {
                ContinuousInterpolation::set_target_direct(&inter, case.0);
            } else {
                ContinuousInterpolation::set_target(&inter, case.0);
            }
        };

        update();
        Timer::repeat(Duration::from_millis(200), move || {
            update();
            true
        });
    }
}
