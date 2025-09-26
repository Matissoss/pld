// pld - src/obj/elf/mod.rs
// ------------------------
// made by matissoss
// licensed under MPL 2.0

pub mod par;
pub mod link;

use crate::cli;

// placeholder struct

pub type Elf = Elf64;

#[derive(Debug)]
pub struct Elf64;
pub struct Elf32;

impl Elf32 {
    pub fn to_64_bit(&self) -> Elf64 {
        todo!()
    }
}

impl Elf {
    // placeholder methods (to be replaced with real)
    pub fn link(_cli: &cli::Cli, _vec: Vec<Elf>) -> Vec<u8> {
        todo!("trololo");
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Elf, String> {
        println!("{:?}", par::parelf(bytes));
        todo!("trololo")
    }
}

// low-level structs

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf32_Ehdr {
    pub magic: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf64_Ehdr {
    pub magic: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf32_Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf32_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf64_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf32_Sym {
    pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub sh_shndx: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf32_Rel {
    pub r_offset: u32,
    pub r_info: u32
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf32_Rela {
    pub r_offset: u32,
    pub r_info: u32,
    pub r_addend: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf64_Rel {
    pub r_offset: u64,
    pub r_info: u64
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}
