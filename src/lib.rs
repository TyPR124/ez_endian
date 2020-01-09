#![forbid(unsafe_code)]
#![allow(non_camel_case_types)]
#![no_std]

#[macro_use]
mod bitwise;
#[macro_use]
mod fmt;
#[macro_use]
mod integers;
// #[macro_use] mod floats;
#[macro_use]
mod math;

#[cfg(test)]
mod tests;

define_integer_quads! {
    i8: i8be, i8le;
    u8: u8be, u8le;

    i16: i16be, i16le;
    u16: u16be, u16le;

    i32: i32be, i32le;
    u32: u32be, u32le;

    i64: i64be, i64le;
    u64: u64be, u64le;

    i128: i128be, i128le;
    u128: u128be, u128le;
}
