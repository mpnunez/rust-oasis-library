use std::io::Write;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::fmt::Debug;

trait ToUnsigned {
    type UnsignedType;
    
    fn to_uns(&self) -> Self::UnsignedType
        where Self::UnsignedType: TryFrom<Self>,
        Self: Sized + Copy,
        <Self::UnsignedType as TryFrom<Self>>::Error: Debug
    {
        Self::UnsignedType::try_from(*self).unwrap()    // TODO return optional or result
    }
}

impl ToUnsigned for i8 {type UnsignedType = u8;}
impl ToUnsigned for i16 {type UnsignedType = u16;}
impl ToUnsigned for i32 {type UnsignedType = u32;}
impl ToUnsigned for i64 {type UnsignedType = u64;}

struct RealNumberType {}

impl RealNumberType {
    pub const POSITIVE_UNS_INT: u8 = 0;
    pub const NEGATIVE_UNS_INT: u8 = 1;
    pub const POSITIVE_RECIPROCAL: u8 = 2;
    pub const NEGATIVE_RECIPRCAL: u8 = 3;
    pub const POSITIVE_RATIO: u8 = 4;
    pub const NEGATIVE_RATIO: u8 = 5;
    pub const SINGLE_FLOAT: u8 = 6;
    pub const DOUBLE_FLOT: u8 = 7;
}

/**
 A b-string (“binary string”) is a string which may contain any
combination of 8-bit character codes in any sequence. An a-string (“ASCII string”) may contain only printable
ASCII character codes (hexadecimal 21-7E) plus the SP (space) character (hexadecimal 20), in any sequence. An
n-string (“name string”) may contain only printable ASCII character codes (hexadecimal 21-7E), and must have a
length greater than zero.
RAW: Write the bytes blindly
*/
pub enum StringType {A,B,N,RAW}

pub trait WriteOasis {
    fn write_uns_int<T2>(&mut self, n: T2) -> std::io::Result<()>
        where T2: num::integer::Integer
        + num::Unsigned
        + std::ops::Shl<i32, Output = T2>
        + std::ops::Shr<i32, Output = T2>
        + Copy
        + TryInto<u8>
        , <T2 as TryInto<u8>>::Error: Debug;
    fn write_sgn_int(&mut self, n: i32) -> std::io::Result<()>;
    fn write_f32(&mut self, n: f32) -> std::io::Result<()>;
    fn write_string(&mut self, s: &str) -> std::io::Result<()>;
}

// https://stackoverflow.com/questions/29256519/i-implemented-a-trait-for-another-trait-but-cannot-call-methods-from-both-traits
impl<T> WriteOasis for T
where T: Write
{
    fn write_uns_int<T2>(&mut self, n: T2) -> std::io::Result<()>
        where T2: num::integer::Integer
        + num::Unsigned
        + std::ops::Shl<i32, Output = T2>
        + std::ops::Shr<i32, Output = T2>
        + Copy
        + TryInto<u8>
        , <T2 as TryInto<u8>>::Error: Debug
    {
        
        const CONTINUE_MASK: u8 = 1 << 7;
        const VALUE_MASK: u8 = !CONTINUE_MASK;

        let mut current_value = n;

        loop {
            let n_next_value = current_value >> 7;
            let n_u8_value = current_value - (n_next_value << 7);
            let n_u8_value_u8: u8 = n_u8_value.try_into()
                .expect("Value does not fit into u8");
                
            let mut next_byte =  n_u8_value_u8 & VALUE_MASK;
            if n_next_value > T2::zero() {
                next_byte = CONTINUE_MASK | next_byte;
            }
            self.write_all(&[next_byte])?;

            if n_next_value == T2::zero() {
                break;
            } else {
                current_value = n_next_value;
            }
        }

        Ok(())
    }

    fn write_f32(&mut self, n: f32) -> std::io::Result<()> {
        self.write_uns_int(RealNumberType::SINGLE_FLOAT)?;
        let bytes = n.to_ne_bytes();
        self.write_all(&bytes)
    }
    fn write_sgn_int(&mut self, n: i32) -> std::io::Result<()>{Ok(())}
    fn write_string(&mut self, s: &str) -> std::io::Result<()>{Ok(())}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_sgn_int_to_uns_int(){
        assert_eq!(5_i8.to_uns(),5_u8);
        assert_eq!(5_i16.to_uns(),5_u16);
        assert_eq!(5_i32.to_uns(),5_u32);
        assert_eq!(5_i64.to_uns(),5_u64);
    }

    #[test]
    fn write_u32(){
        let mut bw = Vec::<u8>::new();
        let bigger: u32 = 128;
        let result = bw.write_uns_int(bigger);
        assert!(result.is_ok());
    }

    #[test]
    fn write_u64(){
        let mut bw = Vec::<u8>::new();
        let bigger = 128_u64;
        let result = bw.write_uns_int(bigger);
        assert!(result.is_ok());
    }

    #[test]
    fn write_sgn_as_uns(){
        let mut bw = Vec::<u8>::new();
        let signed_int = 4000_i32;
        let result = bw.write_uns_int(signed_int.to_uns());
        assert!(result.is_ok());
    }

    #[test]
    fn write_f32(){
        let mut bw = Vec::<u8>::new();
        let num = 8000_f32;
        let result = bw.write_f32(num);
        assert!(result.is_ok());
    }

    #[test]
    fn write_f64(){
        let mut bw = Vec::<u8>::new();
        let num = 8000_f64;
        //let result = num.write_into(&mut bw);
        //assert!(result.is_ok());
    }

}
