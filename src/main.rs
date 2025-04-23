// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

mod oasis_bytes;
mod record_type;
mod write_bytes;
mod geometry;

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


struct OasisRecordWriter<Wot: WriteOasis> {
    bw: Wot,
    precision: f32,
    oasis_type: OasisType,
    byte_ind: usize,
    next_cell_refnum: u64,
    cellname_table_offset: Option<usize>,
    cell_names: Vec::<String>,
}

impl <Wot: WriteOasis> OasisRecordWriter<Wot> {

    pub fn new(ow: Wot, p: f32) -> Self {
        Self { 
            bw: ow,
            precision: p,
            oasis_type: OasisType::STANDARD,
            byte_ind: 0,
            next_cell_refnum: 0,
            cellname_table_offset: None,
            cell_names: Vec::<String>::new(),
        }
    }

    pub fn initialize_file(&mut self) -> std::io::Result<()> {
        self.byte_ind += self.bw.write_magic_bytes(&self.oasis_type)?;
        self.byte_ind += self.bw.write_start_record(&self.precision)?;
        Ok(())
    }

    pub fn write_cell_record(&mut self, name: &str) -> std::io::Result<()> {
        self.byte_ind += self.bw.write_uns_int(RecordType::CELL_BY_REFNUM)?;
        self.byte_ind += self.bw.write_uns_int(self.next_cell_refnum)?;
        self.next_cell_refnum+=1;
        self.cell_names.push(name.to_string());
        Ok(())
    }

    pub fn write_rectangle_record(&mut self) -> std::io::Result<()> {
        self.byte_ind += self.bw.write_uns_int(RecordType::RECTANGLE)?;
        self.byte_ind += self.bw.write_info_byte( // SWHXYRDL
            false, true, true, true, true, false, true, true
        )?;
        self.byte_ind += self.bw.write_uns_int(1)?;
        self.byte_ind += self.bw.write_uns_int(0)?;
        self.byte_ind += self.bw.write_uns_int(100)?;
        self.byte_ind += self.bw.write_uns_int(200)?;
        self.byte_ind += self.bw.write_sgn_int(0)?;
        self.byte_ind += self.bw.write_sgn_int(0)?;
        Ok(())
    }

    pub fn close_file(&mut self) -> std::io::Result<()> {
        self.write_cell_names()?;
        self.write_end_record()?;
        Ok(())
    }

    fn write_cell_names(&mut self) -> std::io::Result<()> {
        self.cellname_table_offset = Some(self.byte_ind);  // TODO: write this in offset table
        for cn in self.cell_names.iter() {
            self.byte_ind += self.bw.write_uns_int(RecordType::CELLNAME_IMPL_REF_NUM)?;
            self.byte_ind += self.bw.write_string(&cn, StringType::N)?;
        }
        Ok(())
    }

    fn write_end_record(&mut self) -> std::io::Result<()> {
        // End record
        self.byte_ind += self.bw.write_uns_int(RecordType::END)?;
        let offset_table: [u8;12] = [0;12];
        self.bw.write_all(&offset_table)?;    // non-strict table offsets
        self.byte_ind += 12; // whatever the size of the offset table is
        let n_bytes_other_end_stuff: usize = 12 + 1 + 1; // offset table + end validation + length of validation pad
        // and validation
        let n_bytes_padding: usize = OasisBytes::END_RECORD_LENGTH - n_bytes_other_end_stuff;
        let validation_pad: Vec<u8> = vec![RecordType::PAD; n_bytes_padding];
        self.byte_ind += self.bw.write_string(std::str::from_utf8(&validation_pad).unwrap(), StringType::B)?;
        self.byte_ind += self.bw.write_uns_int(OasisBytes::END_RECORD_VALIDATION_NONE)?;
        self.bw.flush()?;
        Ok(())
    }

}

fn main() -> std::io::Result<()> {

    let file_name = "test.oas";
    let f = File::create(file_name)?;
    let mut bw = BufWriter::new(f);

    let mut orw = OasisRecordWriter::new(bw, 8000_f32);
    orw.initialize_file()?;
    orw.write_cell_record("topcell")?;
    orw.write_rectangle_record()?;   // TODO: take a Rectangle as argument
    orw.write_cell_record("cell2")?;
    orw.write_cell_record("cell3")?;
    orw.close_file()?;

    println!("{} bytes written.", orw.byte_ind);

    read_oasis_file(file_name)?;

    Ok(())
}
