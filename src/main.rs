// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

mod oasis_bytes;
mod record_type;
mod write_bytes;

use oasis_bytes::{OasisType,OasisBytes};
use record_type::RecordType;
use write_bytes::{WriteOasis,StringType};


fn read_oasis_file(fname: &str) -> std::io::Result<()> {
    let mut file = File::open(fname)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}

trait PointTrait {}

struct Point {
    x: i64,
    y: i64,
}

impl PointTrait for Point {

}

struct Rectangle<PointType: PointTrait> {
    pt1: PointType,
    pt2: PointType,
}

fn main() -> std::io::Result<()> {

    let file_name = "test.oas";
    let oasis_type = OasisType::STANDARD;
    let precision: f32 = 8000_f32;

    let f = File::create(file_name)?;
    let mut bw = BufWriter::new(f);
    let mut byte_ind: usize = 0;

    byte_ind += bw.write_magic_bytes(&oasis_type)?;
    byte_ind += bw.write_start_record(&precision)?;
    
    let mut next_cell_refnum: u64 = 0;
    
    byte_ind += bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
    byte_ind += bw.write_uns_int(next_cell_refnum)?;
    next_cell_refnum+=1;

    // write a rectangle

    byte_ind += bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
    byte_ind += bw.write_uns_int(next_cell_refnum)?;
    next_cell_refnum+=1;

    byte_ind += bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
    byte_ind += bw.write_uns_int(next_cell_refnum)?;
    next_cell_refnum+=1;

    let cellname_table_offset = byte_ind;
    byte_ind += bw.write_uns_int(RecordType::CELLNAME_IMPL_REF_NUM)?;
    byte_ind += bw.write_string("topcell",StringType::N)?;
    byte_ind += bw.write_uns_int(RecordType::CELLNAME_IMPL_REF_NUM)?;
    byte_ind += bw.write_string("cell2",StringType::N)?;
    byte_ind += bw.write_uns_int(RecordType::CELLNAME_IMPL_REF_NUM)?;
    byte_ind += bw.write_string("cell3",StringType::N)?;

    

    // End record
    byte_ind += bw.write_uns_int(RecordType::END)?;
    let offset_table: [u8;12] = [0;12];
    bw.write_all(&offset_table)?;    // non-strict table offsets
    byte_ind += 12; // whatever the size of the offset table is
    let n_bytes_other_end_stuff: usize = 12 + 1 + 1; // offset table + end validation + length of validation pad
    // and validation
    let n_bytes_padding: usize = OasisBytes::END_RECORD_LENGTH - n_bytes_other_end_stuff;
    let validation_pad: Vec<u8> = vec![RecordType::PAD; n_bytes_padding];
    byte_ind += bw.write_string(std::str::from_utf8(&validation_pad).unwrap(), StringType::B)?;
    byte_ind += bw.write_uns_int(OasisBytes::END_RECORD_VALIDATION_NONE)?;
    bw.flush()?;

    println!("{} bytes written.", byte_ind);

    read_oasis_file("test.oas")?;

    Ok(())
}
