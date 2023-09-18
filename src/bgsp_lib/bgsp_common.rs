pub use image::{Rgba ,RgbaImage, imageops};
use crate::{
    x, y,
};

pub const PATTERN_SIZE: usize = 8;
pub const PATTERN_SIZE64: usize = 64;
pub const NUM_PALETTE_TBL: usize = 64;
pub const NUM_PALETTE_COL: usize = 256;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}
impl<T> Pos<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

pub type BgPos = Pos<i32>;
pub type SpPos = Pos<i32>;

type Code = u32;
type Palette = u32;

pub type BgCode = Code;
pub type BgPalette = Palette;
pub type BgSymmetry = Symmetry;

pub type SpCode = Code;
pub type SpPalette = Palette;
pub type SpSymmetry = Symmetry;

const FLIP_H: isize    = 0b001;
const FLIP_V: isize    = FLIP_H << 1; // = 0b010
const ROTATE_90: isize = FLIP_V << 1; // = 0b100
const FLIP_HV: isize           = FLIP_H | FLIP_V;
const ROTATE_90_FLIP_H: isize  = ROTATE_90 | FLIP_H;
const ROTATE_90_FLIP_V: isize  = ROTATE_90 | FLIP_V;
const ROTATE_90_FLIP_HV: isize = ROTATE_90 | FLIP_HV;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Symmetry {
    Normal         = 0b000,
    FlipH          = FLIP_H,
    FlipV          = FLIP_V,
    FlipHV         = FLIP_HV,  // Rotate180
    Rotate90       = ROTATE_90,
    Rotate90FlipH  = ROTATE_90_FLIP_H,
    Rotate90FlipV  = ROTATE_90_FLIP_V,
    Rotate90FlipHV = ROTATE_90_FLIP_HV,  // Rotate270
}

impl Default for Symmetry {
    fn default() -> Self { Self::Normal }
}

impl From<isize> for Symmetry {
    fn from(n: isize) -> Self {
        match n & ROTATE_90_FLIP_HV {
            FLIP_H            => Self::FlipH,
            FLIP_V            => Self::FlipV,
            FLIP_HV           => Self::FlipHV,
            ROTATE_90         => Self::Rotate90,
            ROTATE_90_FLIP_H  => Self::Rotate90FlipH,
            ROTATE_90_FLIP_V  => Self::Rotate90FlipV,
            ROTATE_90_FLIP_HV => Self::Rotate90FlipHV,
            _                 => Self::Normal,
        }
    }
}

impl Symmetry {
    #[allow(non_upper_case_globals)]
    pub const Rotate180: Self = Self::FlipHV;
    #[allow(non_upper_case_globals)]
    pub const Rotate270: Self = Self::Rotate90FlipHV;

    #[inline]
    pub fn compose(&self, s: Self) -> Self {
        match (*self, s) {
            (Self::Normal, n) => n,
            (n, Self::Normal) => n,
            (Self::FlipH | Self::FlipV | Self::Rotate180, n)
                => Self::from(*self as isize ^ n as isize),
            (n, Self::FlipH | Self::FlipV | Self::Rotate180)
                => Self::from(n as isize ^ s as isize),
            _   => Self::from(*self as isize ^ s as isize ^ FLIP_HV),
        }
    }

    #[inline]
    pub fn enable(&self, s: Self) -> Self {
        Self::from(*self as isize | s as isize)
    }

    #[inline]
    pub fn disable(&self, s: Self) -> Self {
        Self::from(*self as isize & !(s as isize))
    }

    #[inline]
    pub fn non_default() -> Self {
        Self::from(Self::default() as isize | ROTATE_90_FLIP_HV)
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn has_flipH(&self) -> bool {
        *self as isize & FLIP_H != 0
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn has_flipV(&self) -> bool {
        *self as isize & FLIP_V != 0
    }

    #[inline]
    pub fn has_rotate90(&self) -> bool {
        *self as isize & ROTATE_90 != 0
    }
}

pub static XY_MAP64: [[[(u32, u32); PATTERN_SIZE64]; PATTERN_SIZE64]; 8] = {
    const Q_OFFSET: i32 = PATTERN_SIZE64 as i32 - 1;
    let mut xy_map: [[[(u32, u32); PATTERN_SIZE64]; PATTERN_SIZE64]; 8] = [[[(0, 0); PATTERN_SIZE64]; PATTERN_SIZE64]; 8];
    #[allow(clippy::type_complexity)]
    const VECTORS_AND_OFFSET: [((i32, i32), (i32, i32), (i32, i32)); 8] = [
        (( 1, 0), ( 0, 1), (       0,        0)), // 0:Normal
        ((-1, 0), ( 0, 1), (Q_OFFSET,        0)), // 1:FlipH
        (( 1, 0), ( 0,-1), (       0, Q_OFFSET)), // 2:FlipV
        ((-1, 0), ( 0,-1), (Q_OFFSET, Q_OFFSET)), // 3:FlipHV(=Rotate180)
        (( 0, 1), (-1, 0), (Q_OFFSET,        0)), // 4:Rotate90
        (( 0, 1), ( 1, 0), (       0,        0)), // 5:Rotate90FlipH
        (( 0,-1), (-1, 0), (Q_OFFSET, Q_OFFSET)), // 6:Rotate90FlipV
        (( 0,-1), ( 1, 0), (       0, Q_OFFSET)), // 7:Rotate90FlipHV(=Rotate270)
    ];
    let mut symmetry = 0;
    while symmetry < 8 {
        let unit_i = VECTORS_AND_OFFSET[symmetry].0;
        let unit_j = VECTORS_AND_OFFSET[symmetry].1;
        let offset = VECTORS_AND_OFFSET[symmetry].2;
        let mut y = 0;
        let mut y_j = (0, 0);
        while y < PATTERN_SIZE64 {
            let mut x = 0;
            let mut x_i = (0, 0);
            while x < PATTERN_SIZE64 {
                let mapped_x = x![x_i] + x![y_j] + x![offset];
                let mapped_y = y![x_i] + y![y_j] + y![offset];
                xy_map[symmetry][y][x] = (mapped_x as u32, mapped_y as u32);
                x += 1;
                x![x_i] += x![unit_i];
                y![x_i] += y![unit_i];
            }
            y += 1;
            x![y_j] += x![unit_j];
            y![y_j] += y![unit_j];
        }
        symmetry += 1;
    }
    xy_map
};

pub fn draw(
    size: (u32, u32),
    pattern: &[u64],
    color_tbl: &[Rgba<u8>],
    symmetry: Symmetry,
    position: (u32, u32),
    scalar: (u32, u32),
    gbuf: &mut image::RgbaImage,
) {
    if x![size] == 0 || y![size] == 0 || x![scalar] == 0 || y![scalar] == 0 {
        return;
    }
    let draw_size = (PATTERN_SIZE as i32 * x![size] as i32, PATTERN_SIZE as i32 * y![size] as i32);
    let (unit_i, unit_j, offset) = match symmetry {
        Symmetry::Normal         => (( 1, 0), ( 0, 1), (                0,                 0)),
        Symmetry::FlipH          => ((-1, 0), ( 0, 1), (x![draw_size] - 1,                 0)),
        Symmetry::FlipV          => (( 1, 0), ( 0,-1), (                0, y![draw_size] - 1)),
        Symmetry::FlipHV         => ((-1, 0), ( 0,-1), (x![draw_size] - 1, y![draw_size] - 1)),
        Symmetry::Rotate90       => (( 0, 1), (-1, 0), (y![draw_size] - 1,                 0)),
        Symmetry::Rotate90FlipH  => (( 0, 1), ( 1, 0), (                0,                 0)),
        Symmetry::Rotate90FlipV  => (( 0,-1), (-1, 0), (y![draw_size] - 1, x![draw_size] - 1)),
        Symmetry::Rotate90FlipHV => (( 0,-1), ( 1, 0), (                0, x![draw_size] - 1)),
    };
    let mut idx = 0;
    let mut y_j = (0, 0);
    for _ in 0..y![draw_size] {
        let mut x_i = (0, 0);
        for _ in 0..x![size] {
            let row = &pattern[idx];
            idx += 1;
            for q in 0..PATTERN_SIZE {
                let c = (*row >> ((7 - q) * 8)) & 0xff;
                let rgba = color_tbl[c as usize];
                let px = x![position] + (x![x_i] + x![y_j] + x![offset]) as u32 * x![scalar];
                let py = y![position] + (y![x_i] + y![y_j] + y![offset]) as u32 * y![scalar];
                for sy in 0..y![scalar] {
                    for sx in 0..x![scalar] {
                        gbuf.put_pixel(px + sx, py + sy, rgba);
                    }
                }
                x![x_i] += x![unit_i];
                y![x_i] += y![unit_i];
            }
        }
        x![y_j] += x![unit_j];
        y![y_j] += y![unit_j];
    }
}

pub fn draw64x64(
    pattern: &[u64],
    color_tbl: &[Rgba<u8>],
    symmetry: Symmetry,
    position: (u32, u32),
    scalar: (u32, u32),
    gbuf: &mut image::RgbaImage,
) {
    if x![scalar] == 0 || y![scalar] == 0 {
        return;
    }
    let mut idx = 0;
    #[allow(clippy::needless_range_loop)]
    for py in 0..PATTERN_SIZE64 {
        let mut px = 0;
        for _ in 0..(PATTERN_SIZE64 / PATTERN_SIZE) {
            let row = &pattern[idx];
            for q in 0..PATTERN_SIZE {
                let (mapped_px, mapped_py) = XY_MAP64[symmetry as usize][py][px];
                let (final_px, final_py) = (x![position] + mapped_px * x![scalar], y![position] + mapped_py * y![scalar]);
                let c = (*row >> ((7 - q) * 8)) & 0xff;
                let rgba = color_tbl[c as usize];
                for sy in 0..y![scalar] {
                    for sx in 0..x![scalar] {
                        gbuf.put_pixel(final_px + sx, final_py + sy, rgba);
                    }
                }
                px += 1;
            }
            idx += 1;
        }
    }
}
