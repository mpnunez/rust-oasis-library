use std::io::Write;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::fmt::Debug;

pub trait WriteToOasis {
    fn write_into(&self, bw: &mut impl Write) -> std::io::Result<()>
        where Self: num::integer::Integer
            + num::Unsigned
            + std::ops::Shl<i32, Output = Self>
            + std::ops::Shr<i32, Output = Self>
            + Copy
            + TryInto<u8>
            , <Self as TryInto<u8>>::Error: Debug
        {
        
        const CONTINUE_MASK: u8 = 1 << 7;
        const VALUE_MASK: u8 = !CONTINUE_MASK;

        let mut current_value = *self;

        loop {
            let n_next_value = current_value >> 7;
            let n_u8_value = current_value - (n_next_value << 7);
            let n_u8_value_u8: u8 = n_u8_value.try_into()
                .expect("Value does not fit into u8");
                
            let mut next_byte =  n_u8_value_u8 & VALUE_MASK;
            if n_next_value > Self::zero() {
                next_byte = CONTINUE_MASK | next_byte;
            }
            bw.write_all(&[next_byte])?;

            if n_next_value == Self::zero() {
                break;
            } else {
                current_value = n_next_value;
            }
        }

        Ok(())
    }
}

impl WriteToOasis for u8 {}
impl WriteToOasis for u16 {}
impl WriteToOasis for u32 {}
impl WriteToOasis for u64 {}

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_1(){
        let mut bw = Vec::<u8>::new();
        let bigger: u32 = 128;
        let result = bigger.write_into(&mut bw);
        assert!(result.is_ok());
    }

    #[test]
    fn write_2(){
        let mut bw = Vec::<u8>::new();
        let signed_int: i32 = 4000;
        let result = signed_int.to_uns().write_into(&mut bw);
        assert!(result.is_ok());
    }

    #[test]
    fn write_sau(){
        assert_eq!(5_i8.to_uns(),5_u8);
        assert_eq!(5_i16.to_uns(),5_u16);
        assert_eq!(5_i32.to_uns(),5_u32);
        assert_eq!(5_i64.to_uns(),5_u64);
    }

}
