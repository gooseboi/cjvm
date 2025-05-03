use std::io::{Cursor, Read};

pub fn read_u8_be(buf: &mut impl Read) -> u8 {
    let mut tmp = [0u8; 1];
    buf.read_exact(&mut tmp).expect("it's a cursor");
    u8::from_be_bytes(tmp)
}

pub fn read_u16_be(buf: &mut impl Read) -> u16 {
    let mut tmp = [0u8; 2];
    buf.read_exact(&mut tmp).expect("it's a cursor");
    u16::from_be_bytes(tmp)
}

pub fn read_u32_be(buf: &mut impl Read) -> u32 {
    let mut tmp = [0u8; 4];
    buf.read_exact(&mut tmp).expect("it's a cursor");
    u32::from_be_bytes(tmp)
}

pub fn read_u64_be(buf: &mut impl Read) -> u64 {
    let mut tmp = [0u8; 8];
    buf.read_exact(&mut tmp).expect("it's a cursor");
    u64::from_be_bytes(tmp)
}

pub fn read_f32_be(buf: &mut impl Read) -> f32 {
    let mut tmp = [0u8; 4];
    buf.read_exact(&mut tmp).expect("it's a cursor");
    let res = u32::from_be_bytes(tmp);
    // Refer to [`ConstantPoolInfo::Float::bytes`].
    match res {
        0x7f80_0000 => f32::INFINITY,
        0xff80_0000 => f32::NEG_INFINITY,
        0x7f80_0001..=0x7fff_ffff | 0xff80_0001..=0xffff_ffff => f32::NAN,
        _ => f32::from_bits(res),
    }
}

pub fn read_f64_be(buf: &mut impl Read) -> f64 {
    let mut tmp = [0u8; 8];
    buf.read_exact(&mut tmp).expect("it's a cursor");
    f64::from_be_bytes(tmp)
}
