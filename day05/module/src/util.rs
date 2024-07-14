use std::{io::{self, Read, Write, BufReader, BufWriter, ErrorKind}, path::Path, fs::File};

pub fn test() {
  println!("test");
}

pub fn copy_file<P: AsRef<Path>>(copy_path: P, target_path: P) -> io::Result<()> {
  let file = File::open(copy_path)?;
  let mut reader = BufReader::new(file);
  let mut target_file = File::create(target_path)?;
  let mut write = BufWriter::new(target_file);
  copy(&mut reader, &mut write)?;
  Ok(())
}

pub fn copy<R: Read, W: Write>(file: &mut R, target_file: &mut W) -> io::Result<u64> {

  const BUFFER_SIZE: usize = 8;
  let mut buf = [0u8; BUFFER_SIZE];
  let mut write_len = 0u64;
  loop {
    let len = match file.read(&mut buf) {
        Ok(0) => return Ok(write_len),
        Ok(len) => len,
        Err(ref e) if e.kind()  == ErrorKind::Interrupted =>  continue,
        Err(e) => return Err(e)
    };
    target_file.write_all(&buf[..len])?;
    write_len += len as u64;
  }


}