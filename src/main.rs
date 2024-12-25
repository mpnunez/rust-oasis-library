use std::fs::File;
use std::io::prelude::*;
use std::fs;

struct RecordType {}

impl RecordType {
    const PAD: u8 = 0;
    const START: u8 = 1;
    const END: u8 = 2;
}

fn read_oasis_file(fname: &str) -> std::io::Result<()> {
    let mut file = File::open(fname)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}

fn main() -> std::io::Result<()> {

    const MAGIC_BYTES: &str = "%SEMI-OASIS\r\n";
    const CURVI_MAGIC_BYTES: &str = "%SEMI-OASIS-CURVILINEAR\r\n";

    let mut f = File::create("test.oas")?;
    f.write_all(MAGIC_BYTES.as_bytes())?;

    read_oasis_file("test.oas")?;

    println!("{}",RecordType::START);
    println!("{}",RecordType::END);

    Ok(())
}
