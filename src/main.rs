// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::io::BufWriter;

struct OasisBytes {}

impl OasisBytes {
    const MAGIC_BYTES: &str = "%SEMI-OASIS\r\n";
    const CURVI_MAGIC_BYTES: &str = "%SEMI-OASIS-CURVILINEAR\r\n";
}

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

    let mut f = File::create("test.oas")?;
    let mut bw = BufWriter::new(f);

    bw.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?;
    bw.write_all(&[RecordType::START])?;
    bw.write_all(&[RecordType::END])?;
    bw.flush()?;    // How often do we need to flush?

    read_oasis_file("test.oas")?;

    Ok(())
}
