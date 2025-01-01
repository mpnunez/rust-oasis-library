// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;



mod oasis_bytes;
use oasis_bytes::OasisBytes;
mod record_type;
use record_type::RecordType;
mod write_bytes;
use write_bytes::{WriteOasis,StringType};


fn read_oasis_file(fname: &str) -> std::io::Result<()> {
    let mut file = File::open(fname)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}

enum OasisType {
    STANDARD,
    CURVILINEAR,
}

fn main() -> std::io::Result<()> {

    let file_name = "test.oas";
    let oasis_type = OasisType::STANDARD;
    let precision: f32 = 8000_f32;

    let f = File::create(file_name)?;
    let mut bw = BufWriter::new(f);
    match oasis_type {
        OasisType::STANDARD =>
            bw.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?,
        OasisType::CURVILINEAR =>
            bw.write_all(OasisBytes::CURVI_MAGIC_BYTES.as_bytes())?,
    }

    // Start record
    bw.write_uns_int(RecordType::START)?;
    bw.write_string(OasisBytes::VERSION_STRING, StringType::A)?;
    bw.write_f32(precision)?;
    bw.write_uns_int(OasisBytes::TABLE_OFFSETS_IN_END_RECORD)?;

    // End record
    bw.write_uns_int(RecordType::END)?;
    let offset_table: [u8;12] = [0;12];
    bw.write_all(&offset_table)?;    // non-strict table offsets
    let n_bytes_other_end_stuff: usize = 13; // need to calculate based on offset table
    // and validation
    let n_bytes_padding: usize = OasisBytes::END_RECORD_LENGTH - n_bytes_other_end_stuff;
    let validation_pad: Vec<u8> = vec![RecordType::PAD; n_bytes_padding];
    bw.write_string(std::str::from_utf8(&validation_pad).unwrap(), StringType::B)?;
    bw.write_uns_int(OasisBytes::END_RECORD_VALIDATION_NONE)?;
    bw.flush()?;

    read_oasis_file("test.oas")?;

    Ok(())
}
