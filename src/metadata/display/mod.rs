pub use characteristics::Characteristics;
pub use chromaticity::Chromaticity;
pub use primary::Primaries;

mod characteristics;
mod chromaticity;
mod primary;

pub const RPU_PQ_MAX: f32 = 4095.0;
pub const CHROMATICITY_EPSILON: f32 = 1.0 / 32767.0;
// pub const RPU_L6_MIN_FACTOR: f32 = 10000.0;

#[rustfmt::skip]
pub const PREDEFINED_COLORSPACE_PRIMARIES: &[[f32; 8]] = &[
    [0.68  , 0.32  , 0.265 , 0.69  , 0.15  , 0.06  , 0.3127 , 0.329  ], //  0, DCI-P3 D65
    [0.64  , 0.33  , 0.30  , 0.60  , 0.15  , 0.06  , 0.3127 , 0.329  ], //  1, BT.709
    [0.708 , 0.292 , 0.170 , 0.797 , 0.131 , 0.046 , 0.3127 , 0.329  ], //  2, BT.2020
    [0.63  , 0.34  , 0.31  , 0.595 , 0.155 , 0.07  , 0.3127 , 0.329  ], //  3, BT.601 NTSC / SMPTE-C
    [0.64  , 0.33  , 0.29  , 0.60  , 0.15  , 0.06  , 0.3127 , 0.329  ], //  4, BT.601 PAL / BT.470 BG
    [0.68  , 0.32  , 0.265 , 0.69  , 0.15  , 0.06  , 0.314  , 0.351  ], //  5, DCI-P3
    [0.7347, 0.2653, 0.0   , 1.0   , 0.0001,-0.077 , 0.32168, 0.33767], //  6, ACES
    [0.73  , 0.28  , 0.14  , 0.855 , 0.10  ,-0.05  , 0.3127 , 0.329  ], //  7, S-Gamut
    [0.766 , 0.275 , 0.225 , 0.80  , 0.089 ,-0.087 , 0.3127 , 0.329  ], //  8, S-Gamut-3.Cine
    [0.693 , 0.304 , 0.208 , 0.761 , 0.1467, 0.0527, 0.3127 , 0.329  ],
    [0.6867, 0.3085, 0.231 , 0.69  , 0.1489, 0.0638, 0.3127 , 0.329  ],
    [0.6781, 0.3189, 0.2365, 0.7048, 0.141 , 0.0489, 0.3127 , 0.329  ],
    [0.68  , 0.32  , 0.265 , 0.69  , 0.15  , 0.06  , 0.3127 , 0.329  ],
    [0.7042, 0.294 , 0.2271, 0.725 , 0.1416, 0.0516, 0.3127 , 0.329  ],
    [0.6745, 0.310 , 0.2212, 0.7109, 0.152 , 0.0619, 0.3127 , 0.329  ],
    [0.6805, 0.3191, 0.2522, 0.6702, 0.1397, 0.0554, 0.3127 , 0.329  ],
    [0.6838, 0.3085, 0.2709, 0.6378, 0.1478, 0.0589, 0.3127 , 0.329  ],
    [0.6753, 0.3193, 0.2636, 0.6835, 0.1521, 0.0627, 0.3127 , 0.329  ],
    [0.6981, 0.2898, 0.1814, 0.7189, 0.1517, 0.0567, 0.3127 , 0.329  ],
];

/// Format: `[id, primary_index, peak_brightness, min_pq, eotf(enum usize), range]`
#[rustfmt::skip]
pub const PREDEFINED_MASTERING_DISPLAYS: &[[usize; 6]] = &[
    [ 7, 0, 4000, 62, 0, 0], // Default: 4000-nit, P3, D65, ST.2084
    [ 8, 2, 4000, 62, 0, 0],
    [20, 0, 1000,  7, 0, 0],
    [21, 2, 1000,  7, 0, 0],
    [30, 0, 2000,  7, 0, 0],
    [31, 2, 2000,  7, 0, 0],
];

// pub const CMV29_MASTERING_DISPLAYS_LIST: &[u8] = &[7, 8, 20, 21, 30, 31];

/// Only HOME targets are included.
///
/// Format: `[id, primary_index, peak_brightness, min_pq, eotf(enum usize), range]`
#[rustfmt::skip]
pub const PREDEFINED_TARGET_DISPLAYS: &[[usize; 6]] = &[
    [   1, 1,  100, 62, 2, 0],
    [  27, 0,  600,  0, 0, 0],
    [  28, 2,  600,  0, 0, 0],
    [  37, 0, 2000,  0, 0, 0],
    [  38, 2, 2000,  0, 0, 0],
    [  48, 0, 1000,  0, 0, 0],
    [  49, 2, 1000,  0, 0, 0],
    [9003, 1,  600,  7, 2, 0], // BETA
];

// pub const CMV29_TARGET_DISPLAYS_LIST: &[u8] = &[1, 27, 28, 37, 38, 48, 49];

const ST2084_Y_MAX: f32 = 10000.0;
const ST2084_M1: f32 = 2610.0 / 16384.0;
const ST2084_M2: f32 = (2523.0 / 4096.0) * 128.0;
const ST2084_C1: f32 = 3424.0 / 4096.0;
const ST2084_C2: f32 = (2413.0 / 4096.0) * 32.0;
const ST2084_C3: f32 = (2392.0 / 4096.0) * 32.0;

pub fn pq2l(pq: f32) -> f32 {
    let y = ((pq.powf(1.0 / ST2084_M2) - ST2084_C1)
        / (ST2084_C2 - ST2084_C3 * pq.powf(1.0 / ST2084_M2)))
    .powf(1.0 / ST2084_M1);

    y * ST2084_Y_MAX
}

pub fn find_target_id(max: usize, primary: usize) -> usize {
    get_display_id(PREDEFINED_TARGET_DISPLAYS, max, primary).unwrap_or(0)
}

fn get_display_id(list: &[[usize; 6]], max_luminance: usize, primary: usize) -> Option<usize> {
    list.iter()
        .find(|t| (**t)[2] == max_luminance && (**t)[1] == primary)
        .map(|d| d[0])
}
