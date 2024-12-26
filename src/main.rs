// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Write;

use std::convert::TryInto;
use std::fmt::Debug;

mod oasis_bytes;
use oasis_bytes::OasisBytes;
mod record_type;
use record_type::RecordType;


fn read_oasis_file(fname: &str) -> std::io::Result<()> {
    let mut file = File::open(fname)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}

fn write_uns_int<T>(
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


fn write_sgn_as_uns_int<T>(
    n: T,
    bw: &mut impl Write
) -> std::io::Result<()>
    where T: num::integer::Integer
        + num::Signed
        + std::fmt::Display
    {
        if n < T::zero() {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Negative number cannot be written as unsigned integer"
                )
            );
        }

        let n_abs = num::abs(n);
        println!("Want to write {}", n_abs);
        Ok(())
    }

fn main() -> std::io::Result<()> {

    let f = File::create("test.oas")?;
    let mut bw = BufWriter::new(f);

    bw.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?;
    write_uns_int(RecordType::START,&mut bw)?;
    write_uns_int(RecordType::END,&mut bw)?;

    // Make these unit tests
    let bigger: u32 = 128;
    write_uns_int(bigger,&mut bw)?;

    let signed_int: i32 = 4000;
    write_sgn_as_uns_int::<i32>(signed_int,&mut bw)?;

    let signed_int_neg: i32 = -4000;
    write_sgn_as_uns_int::<i32>(signed_int_neg,&mut bw)?;

    bw.flush()?;

    read_oasis_file("test.oas")?;

    Ok(())
}
