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
    let mut byte_ind: usize = 0;
    match oasis_type {
        OasisType::STANDARD => {
            bw.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?;
            byte_ind += OasisBytes::MAGIC_BYTES.len()
        }
        OasisType::CURVILINEAR => {
            bw.write_all(OasisBytes::CURVI_MAGIC_BYTES.as_bytes())?;
            byte_ind += OasisBytes::CURVI_MAGIC_BYTES.len()
        }
    }

    // Start record
    byte_ind += bw.write_uns_int(RecordType::START)?;
    byte_ind += bw.write_string(OasisBytes::VERSION_STRING, StringType::A)?;
    byte_ind += bw.write_f32(precision)?;
    byte_ind += bw.write_uns_int(OasisBytes::TABLE_OFFSETS_IN_END_RECORD)?;

    
    let mut next_cell_refnum: u64 = 0;
    
    byte_ind += bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
    byte_ind += bw.write_uns_int(next_cell_refnum)?;
    next_cell_refnum+=1;

    byte_ind += bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
    byte_ind += bw.write_uns_int(next_cell_refnum)?;
    next_cell_refnum+=1;

    byte_ind += bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
    byte_ind += bw.write_uns_int(next_cell_refnum)?;
    next_cell_refnum+=1;

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
