use crate::err::Error;
use crate::reader::{Read, Walker};

impl<'a> Read<'a> for &'a str {
    fn parse_tsv(fields: &mut Walker<'a>) -> Result<Self, Error> {
        fields.next_field()
    }
}

fn hex_digit(c: char) -> Result<u8, Error> {
    Ok(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => return Err(Error),
    })
}

impl<const N: usize> Read<'_> for [u8; N] {
    fn parse_tsv(fields: &mut Walker<'_>) -> Result<Self, Error> {
        let hex_data = fields.next_field()?;
        let mut result = [0; N];
        let mut chars = hex_data.chars();
        for byte in result.iter_mut().take(N) {
            *byte = (hex_digit(chars.next().ok_or(Error)?)? << 4)
                | hex_digit(chars.next().ok_or(Error)?)?;
        }
        Ok(result)
    }
}

macro_rules! impl_for_int {
    ($typename: ident) => {
        impl Read<'_> for $typename {
            fn parse_tsv(fields: &mut Walker<'_>) -> Result<Self, Error> {
                Ok((fields.next_field()?).parse::<$typename>()?)
            }
        }
    };
}

impl_for_int!(i8);
impl_for_int!(i16);
impl_for_int!(i32);
impl_for_int!(i64);
impl_for_int!(i128);
impl_for_int!(u8);
impl_for_int!(u16);
impl_for_int!(u32);
impl_for_int!(u64);
impl_for_int!(u128);