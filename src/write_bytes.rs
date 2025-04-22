use std::io::Write;
use std::convert::TryInto;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};

use num_traits::PrimInt;

use crate::oasis_bytes::{OasisType, OasisBytes};
use crate::record_type::RecordType;


struct RealNumberType {}

impl RealNumberType {
    pub const POSITIVE_UNS_INT: u8 = 0;
    pub const NEGATIVE_UNS_INT: u8 = 1;
    pub const POSITIVE_RECIPROCAL: u8 = 2;
    pub const NEGATIVE_RECIPRCAL: u8 = 3;
    pub const POSITIVE_RATIO: u8 = 4;
    pub const NEGATIVE_RATIO: u8 = 5;
    pub const SINGLE_FLOAT: u8 = 6;
    pub const DOUBLE_FLOAT: u8 = 7;
}


pub enum StringType {B,A,N}



pub trait WriteOasis {

    // define trait to simplify where clause: https://stackoverflow.com/questions/26070559/is-there-any-way-to-create-a-type-alias-for-multiple-traits
    fn write_uns_int<T2>(&mut self, n: T2) -> std::io::Result<usize>
        where T2: PrimInt
        + TryInto<u8>
        , <T2 as TryInto<u8>>::Error: Debug;
    fn write_sgn_int<T2: PrimInt>(&mut self, n: T2) -> std::io::Result<usize>
        where T2: PrimInt
        + TryInto<u8>
        , <T2 as TryInto<u8>>::Error: Debug;
    fn write_f32(&mut self, n: f32) -> std::io::Result<usize>;
    fn write_f64(&mut self, n: f64) -> std::io::Result<usize>;
    fn write_string(&mut self, s: &str, st: StringType) -> std::io::Result<usize>;

    // Write records
    fn write_magic_bytes(&mut self, oasis_type: &OasisType) -> std::io::Result<usize>;
    fn write_start_record(&mut self, precision: &f32) -> std::io::Result<usize>;
}

// https://stackoverflow.com/questions/29256519/i-implemented-a-trait-for-another-trait-but-cannot-call-methods-from-both-traits
impl<T: Write> WriteOasis for T
{
    fn write_uns_int<T2>(&mut self, n: T2) -> std::io::Result<usize>
        where T2: PrimInt
        + TryInto<u8>
        , <T2 as TryInto<u8>>::Error: Debug
    {
        const CONTINUE_MASK: u8 = 1 << 7;
        const VALUE_MASK: u8 = !CONTINUE_MASK;

        let mut current_value = n;
        let mut bytes_written: usize = 0;

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
            bytes_written +=1;

            if n_next_value == T2::zero() {
                break;
            } else {
                current_value = n_next_value;
            }
        }

        Ok(bytes_written)
    }

    fn write_sgn_int<T2: PrimInt>(&mut self, n: T2) -> std::io::Result<usize>
        where T2: PrimInt
            + TryInto<u8>
            , <T2 as TryInto<u8>>::Error: Debug{
        const CONTINUE_MASK: u8 = 1 << 7;
        let is_negative: bool = n < T2::zero();
        const SIGN_MASK: u8 = 1;
        const VALUE_MASK: u8 = !(CONTINUE_MASK | SIGN_MASK);

        let mut current_value = n;
        if is_negative {
            current_value = (current_value << 1) >> 1;
        }

        let n_next_value = current_value >> 6;
        let n_u8_value = current_value - (n_next_value << 6);
        let n_u8_value_u8: u8 = n_u8_value.try_into()
            .expect("Value does not fit into u8");

        let mut next_byte =  (n_u8_value_u8 << 1) & VALUE_MASK;
        if n_next_value > T2::zero() {
            next_byte = CONTINUE_MASK | next_byte;
        }
        if is_negative {
            next_byte = SIGN_MASK | next_byte;
        }
        self.write_all(&[next_byte])?;

        if n_next_value == T2::zero() {
            return Ok(1);
        }
        Ok(1_usize + self.write_uns_int(n_next_value)?)
    }

    fn write_f32(&mut self, n: f32) -> std::io::Result<usize> {
        let mut bytes_written = self.write_uns_int(RealNumberType::SINGLE_FLOAT)?;
        let bytes = n.to_ne_bytes();
        self.write_all(&bytes)?;
        bytes_written += bytes.len();
        Ok(bytes_written)
    }

    fn write_f64(&mut self, n: f64) -> std::io::Result<usize> {
        let mut bytes_written = self.write_uns_int(RealNumberType::DOUBLE_FLOAT)?;
        let bytes = n.to_ne_bytes();
        self.write_all(&bytes)?;
        bytes_written += bytes.len();
        Ok(bytes_written)
    }

    /**
    A b-string (“binary string”) is a string which may contain any
    combination of 8-bit character codes in any sequence. An a-string (“ASCII string”) may contain only printable
    ASCII character codes (hexadecimal 21-7E) plus the SP (space) character (hexadecimal 20), in any sequence. An
    n-string (“name string”) may contain only printable ASCII character codes (hexadecimal 21-7E), and must have a
    length greater than zero.
    */
    fn write_string(&mut self, s: &str, st: StringType) -> std::io::Result<usize>{
        
        let str_len = s.len();
        if matches!(st,StringType::N) && str_len == 0 {
            return Err(Error::new(ErrorKind::WriteZero, "n-strings cannot be empty."));
        }
        let mut bytes_written: usize = 0;
        bytes_written += self.write_uns_int(str_len)?;

        if matches!(st,StringType::N) && s.contains(' ') {
            return Err(Error::new(ErrorKind::InvalidData, "n-strings cannot have spaces."));
        }

        let s_bytes = s.as_bytes();

        // If A or N check that all characters are printable
        if matches!(st,StringType::N) || matches!(st,StringType::A) {
            for c in s_bytes { 
                if *c >= 0x20 && *c <= 0x7E {continue;}
                
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "n-strings and a-string can only contain printable ASCII characters."
                ));
            }
                
        }

        self.write_all(s_bytes)?;
        bytes_written += s_bytes.len();
        Ok(bytes_written)
    }

    fn write_magic_bytes(
        &mut self,
        oasis_type: &OasisType
    ) -> std::io::Result<usize>{

        match oasis_type {
            OasisType::STANDARD => {
                self.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?;
                Ok(OasisBytes::MAGIC_BYTES.len())
            }
            OasisType::CURVILINEAR => {
                self.write_all(OasisBytes::CURVI_MAGIC_BYTES.as_bytes())?;
                Ok(OasisBytes::CURVI_MAGIC_BYTES.len())
            }
        }
    }

    fn write_start_record(
        &mut self,
        precision: &f32
    ) -> std::io::Result<usize>{

        let mut byte_ind: usize = 0;

        // Start record
        byte_ind += self.write_uns_int(RecordType::START)?;
        byte_ind += self.write_string(OasisBytes::VERSION_STRING, StringType::A)?;
        byte_ind += self.write_f32(*precision)?;
        byte_ind += self.write_uns_int(OasisBytes::TABLE_OFFSETS_IN_END_RECORD)?;

        Ok(byte_ind)
    }
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
        assert_eq!(result.unwrap(),2);
    }

    #[test]
    fn write_u64(){
        let mut bw = Vec::<u8>::new();
        let bigger = 128_u64;
        let result = bw.write_uns_int(bigger);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),2);
    }

    #[test]
    fn write_sgn_as_uns(){
        let mut bw = Vec::<u8>::new();
        let signed_int = 4000_i32;
        let result = bw.write_uns_int(signed_int.to_uns());
        assert!(result.is_ok());
    }

    #[test]
    fn write_sgn_as_uns_no_convert(){
        let mut bw = Vec::<u8>::new();
        let signed_int = 4000_i32;
        let result = bw.write_uns_int(signed_int);
        assert!(result.is_ok());
    }

    #[test]
    fn write_uns_int(){
        let mut bw = Vec::<u8>::new();
        let signed_int = 4000_i32;
        let result = bw.write_sgn_int(signed_int);
        assert!(result.is_ok());
    }

    #[test]
    fn write_f32(){
        let mut bw = Vec::<u8>::new();
        let num = 8000_f32;
        let result = bw.write_f32(num);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),5);
    }

    #[test]
    fn write_f64(){
        let mut bw = Vec::<u8>::new();
        let num = 8000_f64;
        //let result = num.write_into(&mut bw);
        //assert!(result.is_ok());
    }

    #[test]
    fn write_empty_nstr(){
        let mut bw = Vec::<u8>::new();
        let s = "";
        let result = bw.write_string(s,StringType::N);
        assert!(result.is_err());
    }

    #[test]
    fn write_nstr_with_space(){
        let mut bw = Vec::<u8>::new();
        let s = "string with spaces";
        let result = bw.write_string(s,StringType::N);
        assert!(result.is_err());
    }

    #[test]
    fn write_nstr_with_nonprintable(){
        let mut bw = Vec::<u8>::new();
        let s = "string\nwith\nnon\nprintable";
        let result = bw.write_string(s,StringType::N);
        assert!(result.is_err());
    }

    #[test]
    fn write_astr_with_nonprintable(){
        let mut bw = Vec::<u8>::new();
        let s = "string\nwith\nnon\nprintable";
        let result = bw.write_string(s,StringType::A);
        assert!(result.is_err());
    }

    #[test]
    fn write_nstr(){
        let mut bw = Vec::<u8>::new();
        let s = "valid_n_string";
        let result = bw.write_string(s,StringType::N);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),1+s.len());
    }

    #[test]
    fn write_astr(){
        let mut bw = Vec::<u8>::new();
        let s = "valid a-string";
        let result = bw.write_string(s,StringType::A);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),1+s.len());
    }

}
