use bgsp_lib2::bgsp_common::{Rgba, NUM_PALETTE_TBL, NUM_PALETTE_COL};

pub const COLOR_TBL: [[Rgba<u8>; NUM_PALETTE_COL]; NUM_PALETTE_TBL] = {
    let mut tbl: [[Rgba<u8>; NUM_PALETTE_COL]; NUM_PALETTE_TBL] = [[Rgba([0, 0, 0, 0]); NUM_PALETTE_COL]; NUM_PALETTE_TBL];
    let mut tbl_no = 0;
    while tbl_no < PALS.len() {
        let pal_data = PALS[tbl_no];
        let mut idx = 0;
        while idx < pal_data.len() {
            let (r, g, b, a) = pal_data[idx];
            tbl[tbl_no][idx] = Rgba([r, g, b, a]);
            idx += 1;
        }
        tbl_no += 1;
    }
    tbl
};

const PALS: &[&[(u8, u8, u8, u8)]] = &[
    &[],
    SP_PAL_1,
    SP_PAL_2,
    SP_PAL_3,
];

const SP_PAL_1: &[(u8, u8, u8, u8)] = &[
    (  0,   0,   0,   0),
    ( 60,  74, 176, 128),
    (152,   6,   6, 128),
    ( 99, 101, 105, 128),
    ( 87,  86,  87, 128),
    (  4,   3,  11, 128),
    ( 40,  49, 157, 128),
    (  9,  12,  69, 128),
    (155, 154, 155, 128),
    ( 86,   5,   4, 128),
    ( 65,  65,  66, 128),
    (125, 126, 139, 128),
    (162,  48,  39, 128),
    ( 36,  38,  44, 128),
    (178, 181, 182, 128),
    (166,  85,  82, 128),
    ( 82,  83,  84, 255),
    (  9,  12,  76, 255),
    (160, 160, 161, 255),
    (164, 164, 165, 255),
    ( 80,  80,  81, 255),
    (  2,   2,   4, 255),
    (167, 167, 168, 255),
    (157, 157, 158, 255),
    (150, 151, 152, 255),
    ( 85,  86,  87, 255),
    (101, 102, 103, 255),
    (147, 147, 148, 255),
    ( 88,  89,  90, 255),
    (154, 154, 155, 255),
    (143, 144, 145, 255),
    (170, 171, 172, 255),
    ( 92,  92,  93, 255),
    ( 13,  14,  14, 255),
    ( 94,  95,  96, 255),
    (139, 139, 140, 255),
    (105, 106, 107, 255),
    (  1,   2,  70, 255),
    ( 97,  98,  99, 255),
    (133, 134, 135, 255),
    ( 76,  76,  77, 255),
    ( 72,  72,  73, 255),
    ( 13,  17,  86, 255),
    ( 29,  38, 141, 255),
    ( 22,  24,  24, 255),
    ( 33,  35,  35, 255),
    ( 43,  43,  43, 255),
    ( 83,   7,   5, 255),
    (  8,  10,  63, 255),
    ( 43,  56, 167, 255),
    ( 52,  66, 175, 255),
    ( 71,   4,   3, 255),
    (110, 111, 112, 255),
    (129, 129, 130, 255),
    ( 49,  55, 148, 255),
    (159,  50,  41, 255),
    ( 54,  54,  57, 255),
    ( 60,  59,  59, 255),
    ( 38,  47, 154, 255),
    ( 69,  68,  68, 255),
    ( 48,  49,  49, 255),
    ( 56,  63, 160, 255),
    ( 62,  71, 170, 255),
    ( 24,  31, 123, 255),
    ( 64,  64,  65, 255),
    ( 16,  23, 108, 255),
    (169,  58,  51, 255),
    (116, 117, 118, 255),
    (146,  41,  33, 255),
    (125, 125, 126, 255),
    (  3,   3,  46, 255),
    ( 79,  24,  18, 255),
    ( 65,  77, 184, 255),
    ( 43,  45, 118, 255),
    (121, 121, 123, 255),
    (175, 176, 177, 255),
    ( 82,  92, 177, 255),
    ( 72,  80, 171, 255),
    ( 25,  31,  90, 255),
    (172,  49,  32, 255),
    ( 95, 102, 177, 255),
    (112,  11,   9, 255),
    ( 91,  39,  35, 255),
    (128,  28,  19, 255),
    (171,  69,  61, 255),
    (109, 114, 176, 255),
    ( 63,  66, 132, 255),
    ( 58,  58,  79, 255),
    ( 39,  41,  66, 255),
    ( 21,  25,  71, 255),
    ( 85,  57,  56, 255),
    (144,  52,  42, 255),
    (173, 130, 130, 255),
    ( 51,   7,   9, 255),
    (170,  82,  75, 255),
    (166,  33,  23, 255),
    (125, 114,  16, 255),
    ( 66,  59,   6, 255),
    (124,  45,  29, 255),
    (157,   3,  10, 255),
    (116, 104,  21, 255),
    (124, 127, 180, 255),
    ( 15,  47, 165, 255),
    (185, 185, 186, 255),
    (170,  99,  93, 255),
    (148,  90,  26, 255),
    (135,  72,  13, 255),
    (107,  73,  73, 255),
    ( 87,  55,  15, 255),
    (132, 128,  19, 255),
    ( 49,   6,  47, 255),
    ( 66,  43,   5, 255),
    (164, 115, 115, 255),
    (140, 102, 103, 255),
    (109,  94,  16, 255),
    ( 95,  97, 143, 255),
    (122,  59, 125, 255),
    (161, 107,  36, 255),
    ( 67,  74,  98, 255),
    (179, 148, 148, 255),
    (109,   4,  55, 255),
    (149,  68,  62, 255),
    (124, 125, 144, 255),
    (149, 141,  25, 255),
    (  1,  19, 160, 255),
    ( 56,  18,  80, 255),
    ( 92,  77,   9, 255),
    (107,  32, 108, 255),
];

const SP_PAL_2: &[(u8, u8, u8, u8)] = &[
    (  0,   0,   0,   0),
    (217,  87,  99, 140),
    (217, 160, 102, 255),
    (238, 195, 154, 255),
    (255, 255, 255, 255),
];

const SP_PAL_3: &[(u8, u8, u8, u8)] = &[
    (  0,   0,   0,   0),
    ( 99,  87, 217, 140),
    (102, 160, 217, 255),
    (154, 195, 238, 255),
    (255, 255, 255, 255),
];
