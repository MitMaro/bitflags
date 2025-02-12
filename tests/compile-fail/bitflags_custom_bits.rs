use std::{
    fmt::{
        self,
        Debug,
        Display,
        LowerHex,
        UpperHex,
        Octal,
        Binary,
    },
    ops::{
        BitAnd,
        BitOr,
        BitXor,
        BitAndAssign,
        BitOrAssign,
        BitXorAssign,
        Not,
    },
};

use bitflags::{bitflags, Bits, parser::{ParseError, WriteHex, ParseHex}};

// Ideally we'd actually want this to work, but currently need something like `num`'s `Zero`
// With some design work it could be made possible
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MyInt(u8);

impl Bits for MyInt {
    const EMPTY: Self = MyInt(u8::MIN);
    const ALL: Self = MyInt(u8::MAX);
}

impl BitAnd for MyInt {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        MyInt(self.0 & other.0)
    }
}

impl BitOr for MyInt {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        MyInt(self.0 | other.0)
    }
}

impl BitXor for MyInt {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        MyInt(self.0 ^ other.0)
    }
}

impl BitAndAssign for MyInt {
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0
    }
}

impl BitOrAssign for MyInt {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

impl BitXorAssign for MyInt {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0
    }
}

impl Not for MyInt {
    type Output = MyInt;

    fn not(self) -> Self {
        MyInt(!self.0)
    }
}

impl Debug for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl LowerHex for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl Octal for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Octal::fmt(&self.0, f)
    }
}

impl Binary for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl ParseHex for MyInt {
    fn parse_hex(input: &str) -> Result<Self, ParseError> {
        Ok(MyInt(u8::from_str_radix(input, 16).map_err(|_| ParseError::invalid_hex_flag(input))?))
    }
}

impl WriteHex for MyInt {
    fn write_hex<W: fmt::Write>(&self, writer: W) -> fmt::Result {
        LowerHex::fmt(&self.0, writer)
    }
}

bitflags! {
    struct Flags128: MyInt {
        const A = MyInt(0b0000_0001u8);
        const B = MyInt(0b0000_0010u8);
        const C = MyInt(0b0000_0100u8);
    }
}

fn main() {}
