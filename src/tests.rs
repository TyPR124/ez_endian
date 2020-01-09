use crate::*;
#[test]
fn le_bits_and_native() {
    let x: u16le = 0x1122.into();
    assert_eq!(u16::to_le(0x1122), x.to_bits());
    assert_eq!(0x1122, x.to_native());
    assert_eq!(x, u16le::from_native(0x1122));

    let x = u16le::from_bits(0x1122);
    assert_eq!(0x1122, x.to_bits());
    assert_eq!(u16::from_le(0x1122), x.to_native());
}
#[test]
fn be_bits_and_native() {
    let x: u16be = 0x1122.into();
    assert_eq!(u16::to_be(0x1122), x.to_bits());
    assert_eq!(0x1122, x.to_native());
    assert_eq!(x, u16be::from_native(0x1122));

    let x = u16be::from_bits(0x1122);
    assert_eq!(0x1122, x.to_bits());
    assert_eq!(u16::from_be(0x1122), x.to_native());
}
#[test]
fn native_endian_is_native_repr() {
    #[cfg(target_endian = "little")]
    let x: u16le = 0x1122.into();
    #[cfg(target_endian = "big")]
    let x: u16be = 0x1122.into();

    assert_eq!(0x1122, x.to_bits());
}
#[test]
fn non_native_endian_is_non_native_repr() {
    #[cfg(target_endian = "little")]
    let x: u16be = 0x1122.into();
    #[cfg(target_endian = "big")]
    let x: u16le = 0x1122.into();

    assert_eq!(0x2211, x.to_bits());
}
#[test]
fn be_binary_ops() {
    let x: u16be = 0x1122.into();
    let y: u16be = 0xF0F0.into();

    assert_eq!(x | y, 0xF1F2.into());
    assert_eq!(x & y, 0x1020.into());
    assert_eq!(x ^ y, 0xE1D2.into());
    assert_eq!(!x, 0xEEDD.into());

    let mut x: u16be = 0x1122.into();
    x |= y;
    assert_eq!(x, 0xF1F2.into());

    let mut x: u16be = 0x1122.into();
    x &= y;
    assert_eq!(x, 0x1020.into());

    let mut x: u16be = 0x1122.into();
    x ^= y;
    assert_eq!(x, 0xE1D2.into());
}
#[test]
fn le_binary_ops() {
    let x: u16le = 0x1122.into();
    let y: u16le = 0xF0F0.into();

    assert_eq!(x | y, 0xF1F2.into());
    assert_eq!(x & y, 0x1020.into());
    assert_eq!(x ^ y, 0xE1D2.into());
    assert_eq!(!x, 0xEEDD.into());

    let mut x: u16le = 0x1122.into();
    x |= y;
    assert_eq!(x, 0xF1F2.into());

    let mut x: u16le = 0x1122.into();
    x &= y;
    assert_eq!(x, 0x1020.into());
    
    let mut x: u16le = 0x1122.into();
    x ^= y;
    assert_eq!(x, 0xE1D2.into());
}
