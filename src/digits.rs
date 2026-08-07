use pebble_rust_2026::GContext;

pub use crate::grid::DigitBounds;
use crate::{
    animation::Status,
    time::{MAX_TIME, TIME_SCALE},
    types::{Dir, Pos, Seg, Stroke, Transition},
};

static TRANSITION_0: Transition = Transition::def(&[Stroke::forward(
    0,
    &[
        Seg::def(Pos::Bl, Dir::U),
        Seg::def(Pos::Tl, Dir::R),
        Seg::def(Pos::Tr, Dir::D),
        Seg::def(Pos::Br, Dir::L),
    ],
)]);

fn render_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_0.render(ctx, bounds, progress)
}

static TRANSITION_1: Transition = Transition::def(&[
    Stroke::forward(0, &[Seg::def(Pos::Bl, Dir::R)]),
    Stroke::forward(
        6 * TIME_SCALE,
        &[Seg::def(Pos::BOne, Dir::U), Seg::def(Pos::TOne, Dir::OneL)],
    ),
]);

fn render_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_1.render(ctx, bounds, progress)
}

static TRANSITION_2: Transition = Transition::def(&[Stroke::forward(
    0,
    &[
        Seg::def(Pos::Tl, Dir::R),
        Seg::def(Pos::Tr, Dir::Hd),
        Seg::def(Pos::Cr, Dir::L),
        Seg::def(Pos::Cl, Dir::Hd),
        Seg::def(Pos::Bl, Dir::R),
    ],
)]);

fn render_2(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_2.render(ctx, bounds, progress)
}

static TRANSITION_3: Transition = Transition::def(&[
    Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tl, Dir::R),
            Seg::def(Pos::Tr, Dir::D),
            Seg::def(Pos::Br, Dir::L),
        ],
    ),
    Stroke::forward(10 * TIME_SCALE, &[Seg::def(Pos::Cr, Dir::L)]),
]);

fn render_3(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_3.render(ctx, bounds, progress)
}

static TRANSITION_4: Transition = Transition::def(&[
    Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tl, Dir::Hd),
            Seg::def(Pos::Cl, Dir::R),
            Seg::def(Pos::Cr, Dir::Hd),
        ],
    ),
    Stroke::forward(9 * TIME_SCALE, &[Seg::def(Pos::Cr, Dir::Hu)]),
]);

fn render_4(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_4.render(ctx, bounds, progress)
}

static TRANSITION_5: Transition = Transition::def(&[Stroke::forward(
    0,
    &[
        Seg::def(Pos::Bl, Dir::R),
        Seg::def(Pos::Br, Dir::Hu),
        Seg::def(Pos::Cr, Dir::L),
        Seg::def(Pos::Cl, Dir::Hu),
        Seg::def(Pos::Tl, Dir::R),
    ],
)]);

fn render_5(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_5.render(ctx, bounds, progress)
}

static TRANSITION_6: Transition = Transition::def(&[Stroke::forward(
    0,
    &[
        Seg::def(Pos::Tr, Dir::L),
        Seg::def(Pos::Tl, Dir::D),
        Seg::def(Pos::Bl, Dir::R),
        Seg::def(Pos::Br, Dir::Hu),
        Seg::def(Pos::Cr, Dir::L),
    ],
)]);

fn render_6(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_6.render(ctx, bounds, progress)
}

static TRANSITION_7: Transition = Transition::def(&[Stroke::forward(
    0,
    &[Seg::def(Pos::Tl, Dir::R), Seg::def(Pos::Tr, Dir::D)],
)]);

fn render_7(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_7.render(ctx, bounds, progress)
}

static TRANSITION_8: Transition = Transition::def(&[
    Stroke::forward(0, &[Seg::def(Pos::Br, Dir::U)]),
    Stroke::forward(
        2 * TIME_SCALE,
        &[Seg::def(Pos::Tr, Dir::L), Seg::def(Pos::Tl, Dir::Hd)],
    ),
    Stroke::forward(2 * TIME_SCALE, &[Seg::def(Pos::Cr, Dir::L)]),
    Stroke::forward(
        2 * TIME_SCALE,
        &[Seg::def(Pos::Br, Dir::L), Seg::def(Pos::Bl, Dir::Hu)],
    ),
]);

fn render_8(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_8.render(ctx, bounds, progress)
}

static TRANSITION_9: Transition = Transition::def(&[
    Stroke::forward(
        0,
        &[
            Seg::def(Pos::Br, Dir::U),
            Seg::def(Pos::Tr, Dir::L),
            Seg::def(Pos::Tl, Dir::Hd),
        ],
    ),
    Stroke::forward(TIME_SCALE * 2, &[Seg::def(Pos::Cr, Dir::L)]),
]);

fn render_9(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    TRANSITION_9.render(ctx, bounds, progress)
}

pub fn render_0_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static TRANSITION: Transition = Transition::def(&[
        Stroke::forward(
            7 * TIME_SCALE,
            &[Seg::def(Pos::BOne, Dir::U), Seg::def(Pos::TOne, Dir::OneL)],
        ),
        Stroke::back(
            0,
            &[
                Seg::def(Pos::Bl, Dir::U),
                Seg::def(Pos::Tl, Dir::R),
                Seg::def(Pos::Tr, Dir::D),
            ],
        ),
    ]);
    static ALWAYS: Transition =
        Transition::def(&[Stroke::forward(0, &[Seg::def(Pos::Bl, Dir::R)])]);
    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_1_2(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(
            0,
            &[Seg::def(Pos::BOne, Dir::U), Seg::def(Pos::TOne, Dir::OneL)],
        ),
        Stroke::forward(
            0,
            &[
                Seg::def(Pos::Bl, Dir::Hu),
                Seg::def(Pos::Cl, Dir::R),
                Seg::def(Pos::Cr, Dir::Hu),
                Seg::def(Pos::Tr, Dir::L),
            ],
        ),
    ]);
    static ALWAYS: Transition =
        Transition::def(&[Stroke::forward(0, &[Seg::def(Pos::Bl, Dir::R)])]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_2_3(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(0, &[Seg::def(Pos::Cl, Dir::Hd), Seg::def(Pos::Bl, Dir::R)]),
        Stroke::forward(
            TIME_SCALE * 3,
            &[Seg::def(Pos::Cr, Dir::Hd), Seg::def(Pos::Br, Dir::L)],
        ),
    ]);
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Cl, Dir::R),
            Seg::def(Pos::Cr, Dir::Hu),
            Seg::def(Pos::Tr, Dir::L),
        ],
    )]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_3_4(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[Seg::def(Pos::Cr, Dir::L), Seg::def(Pos::Tr, Dir::D)],
    )]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(0, &[Seg::def(Pos::Tr, Dir::L)]),
        Stroke::back(0, &[Seg::def(Pos::Br, Dir::L)]),
        Stroke::forward(TIME_SCALE * 3, &[Seg::def(Pos::Cl, Dir::Hu)]),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_4_5(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Cl, Dir::Hu),
            Seg::def(Pos::Cr, Dir::L),
            Seg::def(Pos::Cr, Dir::Hd),
        ],
    )]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(0, &[Seg::def(Pos::Cr, Dir::Hu)]),
        Stroke::forward(0, &[Seg::def(Pos::Tl, Dir::R)]),
        Stroke::forward(0, &[Seg::def(Pos::Br, Dir::L)]),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_5_6(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tl, Dir::R),
            Seg::def(Pos::Cl, Dir::Hu),
            Seg::def(Pos::Cr, Dir::L),
            Seg::def(Pos::Cr, Dir::Hd),
        ],
    )]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(0, &[Seg::def(Pos::Br, Dir::L)]),
        Stroke::forward(
            3 * TIME_SCALE,
            &[Seg::def(Pos::Cl, Dir::Hd), Seg::def(Pos::Bl, Dir::R)],
        ),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_6_7(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition =
        Transition::def(&[Stroke::forward(0, &[Seg::def(Pos::Tl, Dir::R)])]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(
            0,
            &[
                Seg::def(Pos::Tl, Dir::D),
                Seg::def(Pos::Bl, Dir::R),
                Seg::def(Pos::Br, Dir::Hu),
                Seg::def(Pos::Cr, Dir::L),
            ],
        ),
        Stroke::forward(11 * TIME_SCALE, &[Seg::def(Pos::Tr, Dir::D)]),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_7_8(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[Seg::def(Pos::Tl, Dir::R), Seg::def(Pos::Tr, Dir::D)],
    )]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::forward(0, &[Seg::def(Pos::Br, Dir::L), Seg::def(Pos::Bl, Dir::U)]),
        Stroke::forward(0, &[Seg::def(Pos::Cr, Dir::L)]),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_8_9(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tl, Dir::R),
            Seg::def(Pos::Tr, Dir::D),
            Seg::def(Pos::Tl, Dir::Hd),
            Seg::def(Pos::Cl, Dir::R),
        ],
    )]);
    static TRANSITION: Transition = Transition::def(&[Stroke::back(
        0,
        &[Seg::def(Pos::Cl, Dir::Hd), Seg::def(Pos::Bl, Dir::R)],
    )]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_9_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tl, Dir::R),
            Seg::def(Pos::Tr, Dir::D),
            Seg::def(Pos::Tl, Dir::Hd),
        ],
    )]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(0, &[Seg::def(Pos::Cl, Dir::R)]),
        Stroke::forward(
            3 * TIME_SCALE,
            &[Seg::def(Pos::Br, Dir::L), Seg::def(Pos::Bl, Dir::Hu)],
        ),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_1_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(
            0,
            &[Seg::def(Pos::BOne, Dir::U), Seg::def(Pos::TOne, Dir::OneL)],
        ),
        Stroke::forward(
            0,
            &[
                Seg::def(Pos::Bl, Dir::U),
                Seg::def(Pos::Tl, Dir::R),
                Seg::def(Pos::Tr, Dir::D),
            ],
        ),
    ]);
    static ALWAYS: Transition =
        Transition::def(&[Stroke::forward(0, &[Seg::def(Pos::Bl, Dir::R)])]);
    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_5_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static ALWAYS: Transition = Transition::def(&[Stroke::forward(
        0,
        &[Seg::def(Pos::Tl, Dir::R), Seg::def(Pos::Bl, Dir::R)],
    )]);
    static TRANSITION: Transition = Transition::def(&[
        Stroke::back(
            0,
            &[
                Seg::def(Pos::Tl, Dir::Hd),
                Seg::def(Pos::Cl, Dir::R),
                Seg::def(Pos::Cr, Dir::Hd),
            ],
        ),
        Stroke::forward(4 * TIME_SCALE, &[Seg::def(Pos::Br, Dir::U)]),
        Stroke::forward(12 * TIME_SCALE, &[Seg::def(Pos::Bl, Dir::U)]),
    ]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_2_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    static TRANSITION: Transition = Transition::def(&[
        Stroke::forward(
            15 * TIME_SCALE,
            &[Seg::def(Pos::BOne, Dir::U), Seg::def(Pos::TOne, Dir::OneL)],
        ),
        Stroke::back(
            0,
            &[
                Seg::def(Pos::Bl, Dir::Hu),
                Seg::def(Pos::Cl, Dir::R),
                Seg::def(Pos::Cr, Dir::Hu),
                Seg::def(Pos::Tr, Dir::L),
            ],
        ),
    ]);
    static ALWAYS: Transition =
        Transition::def(&[Stroke::forward(0, &[Seg::def(Pos::Bl, Dir::R)])]);

    ALWAYS.render(ctx, bounds, MAX_TIME);
    TRANSITION.render(ctx, bounds, progress)
}

pub fn render_digit(d: i32, ctx: &mut GContext, bounds: &DigitBounds, progress: i32) -> Status {
    (match d {
        0 => render_0,
        1 => render_1,
        2 => render_2,
        3 => render_3,
        4 => render_4,
        5 => render_5,
        6 => render_6,
        7 => render_7,
        8 => render_8,
        9 => render_9,
        _ => return Status::Complete,
    })(ctx, bounds, progress)
}

pub fn render_digit_reverse(
    d: i32,
    ctx: &mut GContext,
    bounds: &DigitBounds,
    mut progress: i32,
) -> Status {
    let transition = match d {
        0 => &TRANSITION_0,
        1 => &TRANSITION_1,
        2 => &TRANSITION_2,
        3 => &TRANSITION_3,
        4 => &TRANSITION_4,
        5 => &TRANSITION_5,
        6 => &TRANSITION_6,
        7 => &TRANSITION_7,
        8 => &TRANSITION_8,
        9 => &TRANSITION_9,
        _ => return Status::Complete,
    };

    let duration = transition.duration();
    if progress >= duration {
        return Status::Complete;
    }
    progress = duration - progress;
    transition.render(ctx, bounds, progress);
    Status::Active
}
