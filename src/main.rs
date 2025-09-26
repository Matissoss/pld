// pld - src/main.rs
// =================
// made by matissoss
// licensed under MPL 2.0

pub mod cli;
pub mod pld;
pub mod obj;
pub mod mmm;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let cli = cli::Cli::from_args(&args);
    let res = pld::link(cli);
    if let Err(why) = res {
        cli::Cli::error(0, why);
    }
}
