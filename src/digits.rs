use pebble_rust_2026::GContext;

pub use crate::grid::DigitBounds;
use crate::{
    time::TIME_SCALE,
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

// pub fn render_0_1(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     static TRANSITION_0_1: Transition = Transition::def(&[]);
//     let mut scratch = 100;
//     bottom(ctx, bounds, &mut scratch, 1);

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_up(),
//             &mut progress,
//             18,
//         );
//         line(
//             ctx,
//             bounds.left_top(),
//             bounds.move_right(),
//             &mut progress,
//             64,
//         );
//         line(
//             ctx,
//             bounds.right_top(),
//             bounds.move_down(),
//             &mut progress,
//             18,
//         );
//         return;
//     }

//     line(
//         ctx,
//         GPoint::new(
//             bounds.base.x + bounds.scale * 6,
//             bounds.base.y + bounds.scale * 2,
//         ),
//         GSize::new(0, -bounds.scale * 2),
//         &mut progress,
//         55,
//     );

//     line(
//         ctx,
//         GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
//         GSize::new(-bounds.scale, 0),
//         &mut progress,
//         45,
//     );
// }

// pub fn render_1_2(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     let mut scratch = 100;
//     bottom(ctx, bounds, &mut scratch, 1);

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             GPoint::new(
//                 bounds.base.x + bounds.scale * 6,
//                 bounds.base.y + bounds.scale * 2,
//             ),
//             GSize::new(0, -bounds.scale * 2),
//             &mut progress,
//             55,
//         );

//         line(
//             ctx,
//             GPoint::new(bounds.base.x + bounds.scale * 6, bounds.base.y),
//             GSize::new(-bounds.scale, 0),
//             &mut progress,
//             45,
//         );
//     } else {
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_half_up(),
//             &mut progress,
//             10,
//         );
//         line(
//             ctx,
//             bounds.left_cross(),
//             bounds.move_right(),
//             &mut progress,
//             40,
//         );
//         line(
//             ctx,
//             bounds.right_cross(),
//             bounds.move_half_up(),
//             &mut progress,
//             10,
//         );
//         line(
//             ctx,
//             bounds.right_top(),
//             bounds.move_left(),
//             &mut progress,
//             40,
//         );
//     }
// }

// pub fn render_2_3(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     let mut scratch = 100;
//     top(ctx, bounds, &mut scratch, 1);
//     cross(ctx, bounds, &mut scratch, 1);
//     bottom(ctx, bounds, &mut scratch, 1);
//     line(
//         ctx,
//         bounds.right_top(),
//         bounds.move_half_down(),
//         &mut scratch,
//         1,
//     );

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_half_up(),
//             &mut progress,
//             100,
//         );
//     } else {
//         line(
//             ctx,
//             bounds.right_cross(),
//             bounds.move_half_down(),
//             &mut progress,
//             100,
//         );
//     }
// }

// pub fn render_3_4(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     let mut scratch = 100;
//     cross(ctx, bounds, &mut scratch, 1);
//     line(ctx, bounds.right_top(), bounds.move_down(), &mut scratch, 1);

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.right_top(),
//             bounds.move_left(),
//             &mut progress,
//             50,
//         );
//         line(
//             ctx,
//             bounds.right_bottom(),
//             bounds.move_left(),
//             &mut progress,
//             50,
//         );
//     } else {
//         line(
//             ctx,
//             bounds.left_cross(),
//             bounds.move_half_up(),
//             &mut progress,
//             50,
//         );
//         line(
//             ctx,
//             bounds.right_cross(),
//             bounds.move_half_down(),
//             &mut progress,
//             50,
//         );
//     }
// }

// pub fn render_4_5(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     let mut scratch = 100;
//     cross(ctx, bounds, &mut scratch, 1);
//     line(
//         ctx,
//         bounds.right_cross(),
//         bounds.move_half_down(),
//         &mut scratch,
//         1,
//     );
//     line(
//         ctx,
//         bounds.left_cross(),
//         bounds.move_half_up(),
//         &mut scratch,
//         1,
//     );

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.right_cross(),
//             bounds.move_half_up(),
//             &mut progress,
//             100,
//         );
//     } else {
//         line(
//             ctx,
//             bounds.left_top(),
//             bounds.move_right(),
//             &mut progress,
//             50,
//         );
//         line(
//             ctx,
//             bounds.right_bottom(),
//             bounds.move_left(),
//             &mut progress,
//             50,
//         );
//     }
// }

// pub fn render_5_6(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     render_5(ctx, bounds, 100);
//     if progress > 0 {
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_half_up(),
//             &mut progress,
//             100,
//         );
//     }
// }

// pub fn render_6_7(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     let mut scratch = 100;
//     line(ctx, bounds.left_top(), bounds.move_right(), &mut scratch, 1);

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.left_top(),
//             bounds.move_down(),
//             &mut progress,
//             11,
//         );
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_right(),
//             &mut progress,
//             42,
//         );
//         line(
//             ctx,
//             bounds.right_bottom(),
//             bounds.move_half_up(),
//             &mut progress,
//             5,
//         );
//         line(
//             ctx,
//             bounds.right_cross(),
//             bounds.move_left(),
//             &mut progress,
//             42,
//         );
//     } else {
//         line(
//             ctx,
//             bounds.right_top(),
//             bounds.move_down(),
//             &mut progress,
//             100,
//         );
//     }
// }

// pub fn render_7_8(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     render_7(ctx, bounds, 100);
//     if progress > 0 {
//         line(
//             ctx,
//             bounds.left_top(),
//             bounds.move_down(),
//             &mut progress,
//             12,
//         );
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_right(),
//             &mut progress,
//             44,
//         );
//         line(
//             ctx,
//             bounds.right_cross(),
//             bounds.move_left(),
//             &mut progress,
//             44,
//         );
//     }
// }

// pub fn render_8_9(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     render_9(ctx, bounds, 100);
//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.right_bottom(),
//             bounds.move_left(),
//             &mut progress,
//             85,
//         );
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_half_up(),
//             &mut progress,
//             15,
//         );
//     }
// }

// pub fn render_9_0(ctx: &mut GContext, bounds: &DigitBounds, mut progress: i32) {
//     let mut scratch = 100;
//     line(ctx, bounds.left_top(), bounds.move_right(), &mut scratch, 1);
//     line(
//         ctx,
//         bounds.left_top(),
//         bounds.move_half_down(),
//         &mut scratch,
//         1,
//     );
//     line(ctx, bounds.right_top(), bounds.move_down(), &mut scratch, 1);

//     if progress < 0 {
//         progress *= -1;
//         line(
//             ctx,
//             bounds.left_cross(),
//             bounds.move_right(),
//             &mut progress,
//             100,
//         );
//     } else {
//         line(
//             ctx,
//             bounds.right_bottom(),
//             bounds.move_left(),
//             &mut progress,
//             89,
//         );
//         line(
//             ctx,
//             bounds.left_bottom(),
//             bounds.move_half_up(),
//             &mut progress,
//             11,
//         );
//     }
// }

// pub fn render_1_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
//     render_0_1(ctx, bounds, -progress);
// }

// pub fn render_5_0(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
//     if progress < 0 {
//         render_5(ctx, bounds, -progress);
//     } else {
//         render_0(ctx, bounds, progress);
//     }
// }

// pub fn render_2_1(ctx: &mut GContext, bounds: &DigitBounds, progress: i32) {
//     render_1_2(ctx, bounds, -progress);
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
