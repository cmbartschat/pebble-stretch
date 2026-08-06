use pebble_rust_2026::GContext;

pub use crate::grid::DigitBounds;
use crate::{
    time::{MAX_TIME, TIME_SCALE},
    types::{Dir, Pos, Seg, Stroke, Transition},
};

fn render_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Bl, Dir::U),
            Seg::def(Pos::Tl, Dir::R),
            Seg::def(Pos::Tr, Dir::D),
            Seg::def(Pos::Br, Dir::L),
        ],
    )]);
    TRANSITION.render(ctx, bounds, progress);
}

fn render_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[
        Stroke::forward(0, &[Seg::def(Pos::Bl, Dir::R)]),
        Stroke::forward(
            6 * TIME_SCALE,
            &[Seg::def(Pos::BOne, Dir::U), Seg::def(Pos::TOne, Dir::OneL)],
        ),
    ]);

    TRANSITION.render(ctx, bounds, progress);
}

fn render_2(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tl, Dir::R),
            Seg::def(Pos::Tr, Dir::Hd),
            Seg::def(Pos::Cr, Dir::L),
            Seg::def(Pos::Cl, Dir::Hd),
            Seg::def(Pos::Bl, Dir::R),
        ],
    )]);

    TRANSITION.render(ctx, bounds, progress);
}

fn render_3(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[
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

    TRANSITION.render(ctx, bounds, progress);
}

fn render_4(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[
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

    TRANSITION.render(ctx, bounds, progress);
}

fn render_5(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Bl, Dir::R),
            Seg::def(Pos::Br, Dir::Hu),
            Seg::def(Pos::Cr, Dir::L),
            Seg::def(Pos::Cl, Dir::Hu),
            Seg::def(Pos::Tl, Dir::R),
        ],
    )]);

    TRANSITION.render(ctx, bounds, progress);
}

fn render_6(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[Stroke::forward(
        0,
        &[
            Seg::def(Pos::Tr, Dir::L),
            Seg::def(Pos::Tl, Dir::D),
            Seg::def(Pos::Bl, Dir::R),
            Seg::def(Pos::Br, Dir::Hu),
            Seg::def(Pos::Cr, Dir::L),
        ],
    )]);

    TRANSITION.render(ctx, bounds, progress);
}

fn render_7(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[Stroke::forward(
        0,
        &[Seg::def(Pos::Tl, Dir::R), Seg::def(Pos::Tr, Dir::D)],
    )]);

    TRANSITION.render(ctx, bounds, progress);
}

fn render_8(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[
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

    TRANSITION.render(ctx, bounds, progress);
}

fn render_9(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
    static TRANSITION: Transition = Transition::def(&[
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

    TRANSITION.render(ctx, bounds, progress);
}

pub fn render_0_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
    TRANSITION.render(ctx, bounds, progress);
}

pub fn render_1_2(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
    TRANSITION.render(ctx, bounds, progress);
}

pub fn render_2_3(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
    TRANSITION.render(ctx, bounds, progress);
}

pub fn render_3_4(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
    TRANSITION.render(ctx, bounds, progress);
}

// pub fn render_4_5(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {

// }

// pub fn render_5_6(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {

// }

// pub fn render_6_7(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {

// }

// pub fn render_7_8(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {

// }

// pub fn render_8_9(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {

// }

// pub fn render_9_0(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {

// }

pub fn render_1_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
    TRANSITION.render(ctx, bounds, progress);
}

// pub fn render_5_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {

// }

// pub fn render_2_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {

// }

pub fn render_digit(d: i32, ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
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
        _ => return,
    })(ctx, bounds, progress)
}
