use core::{cell::RefCell, time::Duration};

use alloc::rc::Rc;
use pebble_rust_2026::{Layer, Timer, resource_ids};

use crate::time::{FRAME_TIME, MAX_TIME, TIME_STEP};

resource_ids!(resource_ids);

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Complete,
    Active,
}

impl Status {
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Complete, Self::Complete) => Self::Complete,
            (Self::Complete, Self::Active) => Self::Active,
            (Self::Active, Self::Complete) => Self::Active,
            (Self::Active, Self::Active) => Self::Active,
        }
    }

    pub fn join(&mut self, other: Self) {
        *self = self.or(other)
    }
}

pub enum InterpolatedDigit {
    Stable(i32),
    Change(i32, i32),
}

fn animate_towards(
    current: (i32, i32, i32, i32),
    next: (i32, i32, i32, i32),
) -> (i32, i32, i32, i32) {
    if current.0 != next.0 {
        if current.3 != -1 {
            return (current.0, current.1, current.2, -1);
        }
        if current.2 != -1 {
            return (current.0, current.1, -1, -1);
        }
        if current.1 != -1 {
            return (current.0, -1, -1, -1);
        }
        return (next.0, -1, -1, -1);
    }

    if current.1 != next.1 {
        if current.3 != -1 {
            return (current.0, current.1, current.2, -1);
        }
        if current.2 != -1 {
            return (current.0, current.1, -1, -1);
        }
        return (current.0, next.1, -1, -1);
    }

    if current.2 != next.2 {
        if current.3 != -1 {
            return (current.0, current.1, current.2, -1);
        }
        return (next.0, next.1, next.2, -1);
    }

    next
}

impl InterpolatedDigit {
    pub fn between(from: i32, to: i32) -> Self {
        if from == to {
            Self::Stable(to)
        } else {
            Self::Change(from, to)
        }
    }

    pub fn end(&self) -> i32 {
        match self {
            Self::Stable(e) => *e,
            Self::Change(_, e) => *e,
        }
    }
}

pub struct InterpolatedTime {
    pub progress: i32,
    pub stage_complete: bool,
    pub digits: (
        InterpolatedDigit,
        InterpolatedDigit,
        InterpolatedDigit,
        InterpolatedDigit,
    ),
}

impl InterpolatedTime {
    pub fn new() -> Self {
        Self {
            progress: MAX_TIME,
            stage_complete: false,
            digits: (
                InterpolatedDigit::Stable(-1),
                InterpolatedDigit::Stable(-1),
                InterpolatedDigit::Stable(-1),
                InterpolatedDigit::Stable(-1),
            ),
        }
    }

    pub fn advance(&mut self, by: i32) {
        self.progress += by;
    }

    pub fn step_towards_direct(&mut self, next: (i32, i32, i32, i32)) {
        let current = (
            self.digits.0.end(),
            self.digits.1.end(),
            self.digits.2.end(),
            self.digits.3.end(),
        );

        if current == next {
            *self = Self {
                progress: MAX_TIME,
                stage_complete: true,
                digits: (
                    InterpolatedDigit::Stable(next.0),
                    InterpolatedDigit::Stable(next.1),
                    InterpolatedDigit::Stable(next.2),
                    InterpolatedDigit::Stable(next.3),
                ),
            };
            return;
        }

        *self = Self {
            progress: 0,
            stage_complete: false,
            digits: (
                InterpolatedDigit::between(current.0, next.0),
                InterpolatedDigit::between(current.1, next.1),
                InterpolatedDigit::between(current.2, next.2),
                InterpolatedDigit::between(current.3, next.3),
            ),
        };
    }

    pub fn step_towards(&mut self, digits: (i32, i32, i32, i32)) {
        let current = (
            self.digits.0.end(),
            self.digits.1.end(),
            self.digits.2.end(),
            self.digits.3.end(),
        );
        let next = animate_towards(current, digits);

        self.step_towards_direct(next);
    }
}

pub struct ContinuousInterpolation {
    layer: Layer,
    pub(crate) time: Rc<RefCell<InterpolatedTime>>,
    target: (i32, i32, i32, i32),
    timer: Option<Timer>,
    finished: bool,
}

impl ContinuousInterpolation {
    pub fn new(layer: Layer, time: Rc<RefCell<InterpolatedTime>>) -> Option<Rc<RefCell<Self>>> {
        let state = Rc::new(RefCell::new(Self {
            layer,
            time,
            target: (-1, -1, -1, -1),
            timer: None,
            finished: true,
        }));

        Some(state)
    }

    pub fn set_target_direct(me: &Rc<RefCell<Self>>, digits: (i32, i32, i32, i32)) {
        let mut state = me.borrow_mut();
        state.target = digits;
        state.finished = false;

        {
            let mut progress = state.time.borrow_mut();
            progress.step_towards_direct(digits);
        }

        if let Some(old_timer) = state.timer.take() {
            old_timer.cancel();
        }

        state.timer = Timer::repeat(Duration::from_millis(FRAME_TIME), {
            let state = me.clone();
            move || {
                let mut state = state.borrow_mut();
                state.layer.mark_dirty();
                {
                    let mut progress = state.time.borrow_mut();
                    if progress.stage_complete {
                        progress.step_towards(state.target);
                    } else {
                        progress.advance(TIME_STEP);
                    }
                    if progress.progress < MAX_TIME {
                        return true;
                    }
                }
                state.finished = true;
                false
            }
        });
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn set_target(me: &Rc<RefCell<Self>>, digits: (i32, i32, i32, i32)) {
        let mut state = me.borrow_mut();
        state.target = digits;
        state.layer.mark_dirty();
        state.finished = false;

        {
            let mut progress = state.time.borrow_mut();
            progress.step_towards(digits);
        }

        if let Some(old_timer) = state.timer.take() {
            old_timer.cancel();
        }

        state.timer = Timer::repeat(Duration::from_millis(FRAME_TIME), {
            let state = me.clone();
            move || {
                let mut state = state.borrow_mut();
                state.layer.mark_dirty();
                {
                    let mut progress = state.time.borrow_mut();
                    if progress.stage_complete {
                        progress.step_towards(state.target);
                    } else {
                        progress.advance(TIME_STEP);
                    }
                    if progress.progress < MAX_TIME {
                        return true;
                    }
                }
                state.finished = true;
                false
            }
        });
    }
}
