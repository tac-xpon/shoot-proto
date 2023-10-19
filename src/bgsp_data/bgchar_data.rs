use bgsp_lib2::bgsp_common::PATTERN_SIZE;

pub const BG_PATTERN_TBL: [Option<(u32, u32, &[u64])>; 256] = {
    let mut tbl: [Option<(u32, u32, &[u64])>; 256] = [None; 256];
    tbl[0x00] = Some((1, 1, &BG_CHARS[0]));
    let mut idx = 0;
    while idx < BG_CHARS.len() {
        tbl[0x20 + idx] = Some((1, 1, &BG_CHARS[idx]));
        idx += 1;
    }
    tbl
};

const BG_CHARS: &[[u64; PATTERN_SIZE]] = &[
    // 0x20 ~ 0x7f(32 ~ 127)
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000001010000,
        0x0000000101010000,
        0x0000000101010000,
        0x0000000101000000,
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
    ],
    [
        0x0001010001010000,
        0x0001010001010000,
        0x0001000001000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000010100010100,
        0x0001010101010101,
        0x0000010100010100,
        0x0000010000010000,
        0x0001010001010000,
        0x0101010101010100,
        0x0001010001010000,
        0x0000000000000000,
    ],
    [
        0x0000000100000000,
        0x0001010101010000,
        0x0101000100000000,
        0x0001010101010000,
        0x0000000100010100,
        0x0101000100010100,
        0x0001010101010000,
        0x0000000100000000,
    ],
    [
        0x0001000000010100,
        0x0100010001010000,
        0x0100010101000000,
        0x0001000100010000,
        0x0000010101000100,
        0x0001010001000100,
        0x0101000000010000,
        0x0000000000000000,
    ],
    [
        0x0000010100000000,
        0x0001000001000000,
        0x0001010001000000,
        0x0000010101000100,
        0x0101000001010100,
        0x0101000000010000,
        0x0001010101000100,
        0x0000000000000000,
    ],
    [
        0x0000010100000000,
        0x0000010100000000,
        0x0000010000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000001000000,
        0x0000000100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000100000000,
        0x0000000001000000,
        0x0000000000000000,
    ],
    [
        0x0000010000000000,
        0x0000000100000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000100000000,
        0x0000010000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000100000000,
        0x0001010101010000,
        0x0000010101000000,
        0x0000010101000000,
        0x0001000000010000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000100000000,
        0x0000000100000000,
        0x0001010101010000,
        0x0000000100000000,
        0x0000000100000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0001010000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000010100,
        0x0000000001010000,
        0x0000000101000000,
        0x0000010100000000,
        0x0001010000000000,
        0x0101000000000000,
        0x0000000000000000,
    ],
    [
        0x0000010101000000,
        0x0001000001010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0001010000010000,
        0x0000010101000000,
        0x0000000000000000,
    ],
    [
        0x0000000101000000,
        0x0000010101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0001010101010100,
        0x0000000000000000,
    ],
    [
        0x0001010101010000,
        0x0101000000010100,
        0x0000000001010100,
        0x0000010101010000,
        0x0001010101000000,
        0x0101010000000000,
        0x0101010101010100,
        0x0000000000000000,
    ],
    [
        0x0001010101010100,
        0x0000000001010000,
        0x0000000101000000,
        0x0000010101010000,
        0x0000000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000101010000,
        0x0000010101010000,
        0x0001010001010000,
        0x0101000001010000,
        0x0101010101010100,
        0x0000000001010000,
        0x0000000001010000,
        0x0000000000000000,
    ],
    [
        0x0101010101010000,
        0x0101000000000000,
        0x0101010101010000,
        0x0000000000010100,
        0x0000000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0001010000000000,
        0x0101000000000000,
        0x0101010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0101010101010100,
        0x0101000000010100,
        0x0000000001010000,
        0x0000000101000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
    ],
    [
        0x0001010101000000,
        0x0101000000010000,
        0x0101010000010000,
        0x0001010101000000,
        0x0100000101010100,
        0x0100000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0001010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0001010101010100,
        0x0000000000010100,
        0x0000000001010000,
        0x0001010101000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0001010000000000,
    ],
    [
        0x0000000001010000,
        0x0000000101000000,
        0x0000010100000000,
        0x0001010000000000,
        0x0000010100000000,
        0x0000000101000000,
        0x0000000001010000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010000,
        0x0000000000000000,
        0x0001010101010000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0001010000000000,
        0x0000010100000000,
        0x0000000101000000,
        0x0000000001010000,
        0x0000000101000000,
        0x0000010100000000,
        0x0001010000000000,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0001010000010100,
        0x0001010000010100,
        0x0000000001010000,
        0x0000000101000000,
        0x0000000000000000,
        0x0000000101000000,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0001000000010100,
        0x0101000101000100,
        0x0100010001000100,
        0x0100010001010100,
        0x0101000101010000,
        0x0001010101010100,
        0x0000000000000000,
    ],
    [
        0x0000010101000000,
        0x0001010001010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101010101010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0000000000000000,
    ],
    [
        0x0101010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101010101010000,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0001010000010100,
        0x0101000000000000,
        0x0101000000000000,
        0x0101000000000000,
        0x0001010000010100,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0101010101000000,
        0x0101000001010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000001010000,
        0x0101010101000000,
        0x0000000000000000,
    ],
    [
        0x0101010101010100,
        0x0101000000000000,
        0x0101000000000000,
        0x0101010101010000,
        0x0101000000000000,
        0x0101000000000000,
        0x0101010101010100,
        0x0000000000000000,
    ],
    [
        0x0101010101010100,
        0x0101000000000000,
        0x0101000000000000,
        0x0101010101010000,
        0x0101000000000000,
        0x0101000000000000,
        0x0101000000000000,
        0x0000000000000000,
    ],
    [
        0x0000010101010100,
        0x0001010000000000,
        0x0101000000000000,
        0x0101000001010100,
        0x0101000000010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101010101010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000000010100,
        0x0000000000010100,
        0x0000000000010100,
        0x0000000000010100,
        0x0000000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101000001010000,
        0x0101000101000000,
        0x0101010100000000,
        0x0101010101000000,
        0x0101000101010000,
        0x0101000001010100,
        0x0000000000000000,
    ],
    [
        0x0101000000000000,
        0x0101000000000000,
        0x0101000000000000,
        0x0101000000000000,
        0x0101000000000000,
        0x0101000000000000,
        0x0101010101010100,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101010001010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0101000100010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101010000010100,
        0x0101010100010100,
        0x0101010101010100,
        0x0101000101010100,
        0x0101000001010100,
        0x0101000000010100,
        0x0000000000000000,
    ],
    [
        0x0001010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0101010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101010101010000,
        0x0101000000000000,
        0x0101000000000000,
        0x0000000000000000,
    ],
    [
        0x0001010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000101010100,
        0x0101000001010000,
        0x0001010101000100,
        0x0000000000000000,
    ],
    [
        0x0101010101010000,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000001010100,
        0x0101010101000000,
        0x0101000101010000,
        0x0101000001010100,
        0x0000000000000000,
    ],
    [
        0x0001010101010000,
        0x0101000000010100,
        0x0101000000000000,
        0x0001010101010000,
        0x0000000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0001010101010100,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101000000010100,
        0x0101000000010100,
        0x0101010001010100,
        0x0001010101010000,
        0x0000010101000000,
        0x0000000100000000,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101000000010100,
        0x0101000100010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0101010001010100,
        0x0101000000010100,
        0x0000000000000000,
    ],
    [
        0x0101000000010100,
        0x0101010001010100,
        0x0001010101010000,
        0x0000010101000000,
        0x0001010101010000,
        0x0101010001010100,
        0x0101000000010100,
        0x0000000000000000,
    ],
    [
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000000000000,
    ],
    [
        0x0101010101010100,
        0x0000000001010100,
        0x0000000101010000,
        0x0000010101000000,
        0x0001010100000000,
        0x0101010000000000,
        0x0101010101010100,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0101000000000000,
        0x0001010000000000,
        0x0000010100000000,
        0x0000000101000000,
        0x0000000001010000,
        0x0000000000010100,
        0x0000000000000000,
    ],
    [
        0x0000010101010000,
        0x0000000001010000,
        0x0000000001010000,
        0x0000000001010000,
        0x0000000001010000,
        0x0000000001010000,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000100000000,
        0x0000010101000000,
        0x0001010001010000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010100,
        0x0000000000000000,
    ],
    [
        0x0000010100000000,
        0x0000000101000000,
        0x0000000001010000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010000,
        0x0000000000010100,
        0x0000010101010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000000000000,
    ],
    [
        0x0001010000000000,
        0x0001010000000000,
        0x0001010101010000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010100,
        0x0001010000000000,
        0x0001010000000000,
        0x0001010000000000,
        0x0000010101010100,
        0x0000000000000000,
    ],
    [
        0x0000000000010100,
        0x0000000000010100,
        0x0000010101010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010000,
        0x0001010000010100,
        0x0001010101010100,
        0x0001010000000000,
        0x0000010101010100,
        0x0000000000000000,
    ],
    [
        0x0000000101010100,
        0x0000010100000000,
        0x0000010100000000,
        0x0001010101010100,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000000010100,
        0x0000010101010000,
    ],
    [
        0x0001010000000000,
        0x0001010000000000,
        0x0001010000000000,
        0x0001010101010000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000000000000000,
    ],
    [
        0x0000000101000000,
        0x0000000000000000,
        0x0000010101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000000010100,
        0x0000000000000000,
        0x0000000000010100,
        0x0000000000010100,
        0x0000000000010100,
        0x0001010000010100,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0001010000000000,
        0x0001010000000000,
        0x0001010000010100,
        0x0001010001010000,
        0x0001010101000000,
        0x0001010101010000,
        0x0001010000010100,
        0x0000000000000000,
    ],
    [
        0x0000010101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000101010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0101010101010000,
        0x0101000100010100,
        0x0101000100010100,
        0x0101000100010100,
        0x0101000100010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010101010000,
        0x0001010000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000000010100,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010000,
        0x0001010000010100,
        0x0001010000000000,
        0x0001010000000000,
        0x0001010000000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0000010101010100,
        0x0001010100000000,
        0x0000010101010000,
        0x0000000001010100,
        0x0001010101010000,
        0x0000000000000000,
    ],
    [
        0x0000010100000000,
        0x0000010100000000,
        0x0001010101010100,
        0x0000010100000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000101010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010000,
        0x0000000101000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0101000100010100,
        0x0101000100010100,
        0x0101000100010100,
        0x0101010101010100,
        0x0001010001010000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010000010100,
        0x0000010101010000,
        0x0000000101000000,
        0x0000010101010000,
        0x0001010000010100,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010000010100,
        0x0001010000010100,
        0x0001010000010100,
        0x0000010101010100,
        0x0000000001010000,
        0x0001010101000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010101010100,
        0x0000000001010100,
        0x0000000101010000,
        0x0000010101000000,
        0x0001010101010100,
        0x0000000000000000,
    ],
    [
        0x0000000101010000,
        0x0000010100000000,
        0x0000010100000000,
        0x0001010000000000,
        0x0000010100000000,
        0x0000010100000000,
        0x0000000101010000,
        0x0000000000000000,
    ],
    [
        0x0000000100000000,
        0x0000000100000000,
        0x0000000100000000,
        0x0000000100000000,
        0x0000000100000000,
        0x0000000100000000,
        0x0000000100000000,
        0x0000000000000000,
    ],
    [
        0x0001010100000000,
        0x0000000101000000,
        0x0000000101000000,
        0x0000000001010000,
        0x0000000101000000,
        0x0000000101000000,
        0x0001010100000000,
        0x0000000000000000,
    ],
    [
        0x0000000000000000,
        0x0000000000000000,
        0x0001010000000100,
        0x0101000100010100,
        0x0100000001010000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
    [
        0x0101010101010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0101010101010100,
        0x0000000000000000,
    ],
];
