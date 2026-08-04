use pebble_rust_2026::{Time, resource_ids};

use crate::format::get_digits;

resource_ids!(resource_ids);

pub enum InterpolatedDigit {
    Stable(i32),
    Change(i32, i32),
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
    pub(crate) progress: i32,
    pub(crate) digits: (
        InterpolatedDigit,
        InterpolatedDigit,
        InterpolatedDigit,
        InterpolatedDigit,
    ),
}

impl InterpolatedTime {
    pub fn new(time: Time) -> Self {
        let digits = get_digits(&time.to_local());
        Self {
            progress: 0,
            digits: (
                InterpolatedDigit::Change(-1, digits.0),
                InterpolatedDigit::Change(-1, digits.1),
                InterpolatedDigit::Change(-1, digits.2),
                InterpolatedDigit::Change(-1, digits.3),
            ),
        }
    }

    pub fn advance(&mut self, by: i32) -> bool {
        self.progress += by;
        self.progress < 100
    }

    pub fn animate_to_time(&mut self, time: Time) {
        let digits = get_digits(&time.to_local());
        self.animate_to_digits(digits);
    }

    pub fn animate_to_digits(&mut self, digits: (i32, i32, i32, i32)) {
        *self = Self {
            progress: -100,
            digits: (
                InterpolatedDigit::between(self.digits.0.end(), digits.0),
                InterpolatedDigit::between(self.digits.1.end(), digits.1),
                InterpolatedDigit::between(self.digits.2.end(), digits.2),
                InterpolatedDigit::between(self.digits.3.end(), digits.3),
            ),
        }
    }
}
