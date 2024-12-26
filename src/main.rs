// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Write;

use std::mem::size_of;
use std::convert::TryInto;
use std::fmt::Debug;

struct OasisBytes {}

impl OasisBytes {
    const MAGIC_BYTES: &'static str = "%SEMI-OASIS\r\n";
    const CURVI_MAGIC_BYTES: &'static str = "%SEMI-OASIS-CURVILINEAR\r\n";
    const VERSION_STRING: &'static str = "1.0";
}

struct RecordType {}


impl RecordType {
    const PAD: u8 = 0;
    const START: u8 = 1;
    const END: u8 = 2;
    const CELLNAME_IMPL_REF_NUM: u8 = 3;
    const CELLNAME_EXPL_REF_NUM: u8 = 4;
    const TEXT_STRING_IMPL_REF_NUM: u8 = 5;
    const TEXT_STRING_EXPL_REF_NUM: u8 = 6;
    const PROPNAME_STRING_IMPL_REF_NUM: u8 = 7;
    const PROPNAME_STRING_EXPL_REF_NUM: u8 = 8;
    const PROPSTRING_IMPL_REF_NUM: u8 = 9;
    const PROPSTRING_EXPL_REF_NUM: u8 = 10;
    /*
    const : u8 = 11;
    const : u8 = 12;
    const : u8 = 13;
    const : u8 = 14;
    const : u8 = 15;
    const : u8 = 16;
    const : u8 = 17;
    const : u8 = 18;
    const : u8 = 19;
    const RECTANGLE: u8 = 20;
    const : u8 = 21;
    const : u8 = 22;
    const : u8 = 23;
    const : u8 = 24;
    const : u8 = 25;
    const : u8 = 26;
    const : u8 = 27;
    const : u8 = 28;
    const : u8 = 29;
    const : u8 = 30;
    const : u8 = 31;
    const : u8 = 32;
    const : u8 = 33;
    const : u8 = 34;
    const MULTIGON: u8 = 35;
    */

}

fn read_oasis_file(fname: &str) -> std::io::Result<()> {
    let mut file = File::open(fname)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}

// see https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
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

fn main() -> std::io::Result<()> {

    let f = File::create("test.oas")?;
    let mut bw = BufWriter::new(f);

    bw.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?;
    write_uns_int(RecordType::START,&mut bw)?;
    write_uns_int(RecordType::END,&mut bw)?;

    let bigger: u32 = 128;
    write_uns_int(bigger,&mut bw)?;
    bw.flush()?;

    read_oasis_file("test.oas")?;

    Ok(())
}
