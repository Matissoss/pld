// pld - src/obj/elf/par.rs
// ------------------------
// made by matissoss
// licensed under MPL 2.0

use crate::{
    obj::elf::*,
    mmm::FromPtr,
};

#[derive(Debug)]
pub enum ElfParError {
    TooSmallInput,  // technically not an error, but these are my rules
    InvalidMag,     // first 4 bytes are invalid. They must be [0x7F, 'E', 'L', 'F']
    EmptyClass,     // byte of idx 5 in Ehdr is 0
    InvalidClass,   // byte of idx 5 in Ehdr is not 1 or 2
    InvalidElfData, // byte of idx 6 in Ehdr is not 1 or 2
    InvalidVersion, // byte of idx 7 in Ehdr is not 1
}

pub fn parelf(elf: &[u8]) -> Result<Elf, ElfParError>{
    use ElfParError::*;
    // must be bigger than ELF magic size
    if elf.len().lt(&16) {
        return Err(TooSmallInput);
    }
    if &elf[0..4] != &[0x7F, 'E' as u8, 'L' as u8, 'F' as u8] {
        return Err(InvalidMag);
    }
    match elf[4] {
        0 => return Err(EmptyClass),
        // ELFCLASS32
        1 => return Ok(parelf32(elf)?.to_64_bit()),
        // ELFCLASS64
        2 => return parelf64(elf),
        _ => return Err(InvalidClass),
    }
}


pub fn parelf32(elf: &[u8]) -> Result<Elf32, ElfParError> {
    use ElfParError::*;
    let _elf_le = match elf[5] {
        1 => true,
        2 => false,
        _ => return Err(InvalidElfData),
    };
    if elf[6] != 1 {
        return Err(InvalidVersion);
    }
    if elf.len().lt(&size_of::<Elf32_Ehdr>()) {
        return Err(TooSmallInput);
    }
    let ehdr = unsafe {
        Elf32_Ehdr::from_ptr(elf.as_ptr()) 
    };
    // now we'll fetch all the needed things

    // first we check if our file is large enough
    if (elf.len() - (ehdr.e_shoff as usize)).lt(&((ehdr.e_shnum as usize) * (ehdr.e_shentsize as usize))) {
        return Err(TooSmallInput);
    }
    // then we make slice out of it
    let shdrs: &[Elf32_Shdr] = unsafe {
        let ptr = std::mem::transmute::<*const u8, *const Elf32_Shdr>(elf.as_ptr().add(ehdr.e_shoff as usize));
        std::slice::from_raw_parts(ptr, ehdr.e_shnum as usize)
    };
    println!("{:?}", ehdr);
    for shdr in shdrs {
        println!("{:?}", shdr);
    }

    todo!()
}

// we will leave 64-bit ELF for later
pub fn parelf64(_elf: &[u8]) -> Result<Elf64, ElfParError> {
    todo!()
}
