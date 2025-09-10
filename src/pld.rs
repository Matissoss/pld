// pld - src/pld.rs
// ----------------
// made by matissoss
// licensed under MPL 2.0

use crate::obj::{
    elf
};
use crate::cli;

pub fn link(cli: cli::Cli) -> Result<(), String> {
    let mut elfs: Vec<elf::Elf> = Vec::new();
    let mut files: Vec<Vec<u8>> = Vec::new();
    for inp in &cli.input {
        let file = std::fs::read(inp);
        match file {
            Ok(v) => files.push(v),
            Err(e) => cli::Cli::error(2, format!("error occured during reading {inp} file: {e}")),
        }
    }
    for file in &files {
        elfs.push(elf::Elf::from_bytes(file)?);
    }
    let bin = elf::Elf::link(&cli, elfs);
    std::fs::write(&*cli.output, bin).map_err(|s| s.to_string())
}
