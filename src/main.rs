use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::BufReader;
use std::io::Read;
use std::cmp;
use std::str::FromStr;
use std::collections::BTreeMap;

extern crate modfile;
use modfile::ptmf;

extern crate rustc_serialize;
extern crate docopt;
use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static USAGE: &'static str = "
mod2abc.

Usage: 
    mod2abc (-h | --help)
    mod2abc (-V | --version)
    mod2abc --in=<filename> --out=<filename>

Options:
    -V, --version         Show version info.
    -h, --help            Show this text.
	--in=<filename>       Name of inputfile
	--out=<filename>      Name of outputfile
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_help: bool,
	flag_version: bool,
	
	flag_in: String,
	flag_out: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
//    println!("{:?}", args);	
	
	if args.flag_version {
		println!("Version: {}", VERSION);
		return;
	}
	
	if args.flag_in == "" {
		println!("No inputfile specificed");
		return
	}

	if args.flag_out == "" {
		println!("No outputfile specificed");
		return
	}

	let ref input_filename = args.flag_in;
	let file = match File::open(input_filename) {
		Ok(file) => file,
		Err(e) => {
			println!("Failed to open file: '{}' Error: '{}'", input_filename, e);
			return
		}
	};

	let read_fn:fn (&mut Read) -> Result<ptmf::PTModule, ptmf::PTMFError> = ptmf::read_mod;

	let mut reader = BufReader::new(&file);
	let module = match read_fn(&mut reader) {
		Ok(module) => module,
		Err(e) => {
			println!("Failed to parse file: '{}' Error: '{:?}'", input_filename, e);
			return;
		}
	};

	let mut pattern_no = 0;
	for pattern in module.patterns {
		println!("song_pattern_{}:", pattern_no);
		pattern_no += 1;
		for row in pattern.rows {
			let ref channel = row.channels[0];
			let number = channel.sample_number as i8 - 1;
			println!("\tdefb {}", number);
		}
		println!("");
	}
	
	
	println!("Done!");
}
