// pld - src/cli.rs
// ================
// made by matissoss
// licensed under MPL 2.0

pub fn version() -> ! {
    println!("pld v<VERSION>");
    std::process::exit(0);
}

pub fn help(bin: &str) -> ! {
    println!("pld v<VERSION>");
    println!("== USAGE ==");
    println!("{bin} [FLAGS] <-i FLAG> <-o FLAG> <-f FLAG>");
    println!("== FLAGS ==");
    println!("-h: display this");
    println!("-v: version");
    println!("-i=<FILE>[,<FILE>][...] - pass input files into linker");
    println!("-o=<PATH> - pass path as output");
    println!("-f=<FORMAT> - specify output format");
    println!("-oreloc - if present, output file will be relocatable");
    println!("-lf - print supported formats");
    std::process::exit(0);
}

#[derive(Default)]
pub struct CliFlags {
    output_reloc: bool,
    // TODO:
    // output_type: BinType,
}

#[derive(Default)]
pub struct Cli<'a> {
    pub input: Vec<&'a str>,
    pub output: Box<str>,
    pub flags: CliFlags,
}

impl<'a> Cli<'a> {
    pub fn error(ecd: u16, msg: impl std::fmt::Display) -> ! {
        eprintln!("error[{ecd:04}]: {msg}");
        std::process::exit(1);
    }
    pub fn from_args(args: &'a [String]) -> Self {
        let mut nself = Self::default();
        for (i, a) in args.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some((key, val)) = a.split_once('=') {
                if val.is_empty() {
                    Self::error(1, format!("empty keyvalue flag: `{a}`"))
                }
                // -key=value flags
                match key {
                    "-i" => nself.input = val.split(',').collect(),
                    "-o" => nself.output = val.into(),
                    _ => Self::error(0, format!("unknown keyvalue flag: `{a}`")),
                }
            } else {
                // -value flags
                match a.as_str() {
                    "-h" => help(&args[0]),
                    "-v" => version(),
                    "-oreloc" => nself.flags.output_reloc = true,
                    _ => Self::error(0, format!("unknown option/flag: `{a}`")),
                }
            }
        }
        nself
    }
}
