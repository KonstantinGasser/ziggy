use lazy_static::lazy_static;

pub type Pattern = Vec<(usize, usize)>;

lazy_static! {
    // Pattern formation:
    //
    //
    // -----x-xx---
    // ----x------x
    // ---xx---x--x
    // xx-x-----xx-
    // xx-x-----xx-
    // ---xx---x--x
    // ----x------x
    // -----x-xx---
   pub static ref COPPERHEAD: Pattern = vec![
        (0, 5),
        (0, 7),
        (0, 8),
        (1, 4),
        (1, 11),
        (2, 3),
        (2, 4),
        (2, 8),
        (2, 11),
        (3, 0),
        (3, 1),
        (3, 3),
        (3, 9),
        (3, 10),
        (4, 0),
        (4, 1),
        (4, 3),
        (4, 9),
        (4, 10),
        (5, 3),
        (5, 4),
        (5, 8),
        (5, 11),
        (6, 4),
        (6, 11),
        (7, 5),
        (7, 7),
        (7, 8)
    ];
}

pub fn copperhead() -> Pattern {
    COPPERHEAD.to_vec()
}
