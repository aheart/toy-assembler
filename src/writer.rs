use crate::elf::{align, Elf};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::mem::size_of;

pub fn write_binary(elf: &Elf, machine_code: &[u8], data: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("executable")?;

    let elf_bytes: &[u8] = unsafe { any_as_u8_slice(&elf) };
    file.write_all(elf_bytes)?;

    let code_offset = align(size_of::<Elf>(), 8);
    file.seek(SeekFrom::Start(code_offset as u64))?;
    file.write_all(&machine_code)?;

    file.seek(SeekFrom::Start(elf.program_headers[1].segment_offset))?;
    file.write_all(data.as_bytes())?;

    Ok(())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}
