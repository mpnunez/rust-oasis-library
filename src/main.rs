// Disk io
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;



mod oasis_bytes;
use oasis_bytes::OasisBytes;
mod record_type;
use record_type::RecordType;
mod write_bytes;
use write_bytes::write_uns_int;


fn read_oasis_file(fname: &str) -> std::io::Result<()> {
    let mut file = File::open(fname)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}



fn main() -> std::io::Result<()> {

    let f = File::create("test.oas")?;
    let mut bw = BufWriter::new(f);

    bw.write_all(OasisBytes::MAGIC_BYTES.as_bytes())?;
    write_uns_int(RecordType::START,&mut bw)?;
    write_uns_int(RecordType::END,&mut bw)?;
    bw.flush()?;

    read_oasis_file("test.oas")?;

    Ok(())
}
