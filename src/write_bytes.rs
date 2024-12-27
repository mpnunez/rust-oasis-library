use std::io::Write;
use std::convert::TryInto;
use std::fmt::Debug;

pub fn write_uns_int<T>(
    n: T,
    bw: &mut impl Write
) -> std::io::Result<()>
    where T: num::integer::Integer
        + num::Unsigned
        + std::ops::Shl<i32, Output = T>
        + std::ops::Shr<i32, Output = T>
        + Copy
        + TryInto<u8>
        , <T as TryInto<u8>>::Error: Debug
        
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
        if n_next_value > T::zero() {
            next_byte = CONTINUE_MASK | next_byte;
        }
        bw.write_all(&[next_byte])?;

        if n_next_value == T::zero() {
            break;
        } else {
            current_value = n_next_value;
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_1(){
        let mut bw = Vec::<u8>::new();
        let bigger: u32 = 128;
        let result = write_uns_int(bigger,&mut bw);
        assert!(result.is_ok());
    }

    #[test]
    fn write_2(){
        let mut bw = Vec::<u8>::new();
        let signed_int: i32 = 4000;
        let result = write_uns_int(u32::try_from(signed_int).unwrap(),&mut bw);
        assert!(result.is_ok());
    }

    #[test]
    fn write_3(){
        let signed_int_neg: i32 = -4000;
        assert!(u32::try_from(signed_int_neg).is_err());
    }
}
