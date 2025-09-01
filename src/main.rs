// pld - src/main.rs
// =================
// made by matissoss
// licensed under STH

pub mod cli;
pub mod pld;
pub mod obj;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let cli = cli::Cli::from_args(&args);
    let res = pld::link(cli);
    if let Err(why) = res {
        cli::Cli::error(0, why);
    }
}
