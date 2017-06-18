use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::BufReader;
use std::io::Read;

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
    mod2abc --in=<filename> --out=<filename> [--only-data]

Options:
    -V, --version         Show version info.
    -h, --help            Show this text.
	--in=<filename>       Name of inputfile
	--out=<filename>      Name of outputfile
	--only-data           Only output rows with data
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_help: bool,
	flag_version: bool,
	
	flag_in: String,
	flag_out: String,
	flag_only_data: bool,
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

	let ref output_filename = args.flag_out;
	let file = match File::create(&output_filename) {
		Ok(file) => file,
		Err(e) => {
			println!("Failed to open file: '{}' Error: '{:?}'", output_filename, e);
			return
		}
	};

	let mut writer = BufWriter::new(&file);		


	// Write in which order to play patterns
	write!(writer, "song_pattern_list:\n").unwrap();
	for i in 0..module.length {
		let position = module.positions.data[i as usize];
		write!(writer, "\tdefw song_pattern_{}\n", position).unwrap();
	}
	// Terminate list
	write!(writer, "\tdefw 0\n").unwrap();
	write!(writer, "\n").unwrap();
	
	let only_data = args.flag_only_data;

	// Write pattern data
	if only_data {
		let mut pattern_no = 0;
		for pattern in module.patterns {
			write!(writer,"song_pattern_{}:\n", pattern_no).unwrap();
			pattern_no += 1;
			let mut row_no = 63;
			let mut last_write = 64;
			for row in pattern.rows {
				let mut number = -1;
				for channel in row.channels {
					if channel.sample_number > 0 {
						number = channel.sample_number as i8 - 1;
						break;
					}
				}
				// Check if we should output anything
				if number == -1 {
					row_no -= 1;
				} else {
					write!(writer,"\tdefb {},{}\n", row_no, number).unwrap();
					last_write = row_no;
					row_no -= 1;
				}
			}
			// Make sure we wait the entire pattern
			if last_write != 0 {
				// Add a row that should never occur
				let number = -1;
				let row_no = -1;
				write!(writer,"\tdefb {},{}\n", row_no, number).unwrap();				
			}
			write!(writer,"\n").unwrap();
		}

	} else {
		// One byte for each row in the pattern,
		// -1 used to signal that nothing happens
		let mut pattern_no = 0;
		for pattern in module.patterns {
			write!(writer,"song_pattern_{}:\n", pattern_no).unwrap();
			pattern_no += 1;
			for row in pattern.rows {
				let mut number = -1;
				for channel in row.channels {
					if channel.sample_number > 0 {
						number = channel.sample_number as i8 - 1;
						break;
					}
				}
				write!(writer,"\tdefb {}\n", number).unwrap();
			}
			write!(writer,"\n").unwrap();
		}
	}

	println!("Done!");
}
