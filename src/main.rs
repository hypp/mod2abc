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
    mod2abc --in=<filename> --out=<filename> [--format-jbe]

Options:
    -V, --version         Show version info.
    -h, --help            Show this text.
	--in=<filename>       Name of inputfile
	--out=<filename>      Name of outputfile
	--format-jbe          Use format for jbe replay
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_help: bool,
	flag_version: bool,
	
	flag_in: String,
	flag_out: String,
	flag_format_jbe: bool
}

fn note_from_period(period: u16) -> String {
	// Find the position in PERIODS with the
	// smallest difference
	let mut found:i32 = -1;
	let mut min_diff = 65536;
	let key = period as i32;
	for i in 0..ptmf::PERIODS.len() {
		let diff = (key as i32 - ptmf::PERIODS[i] as i32).abs();
		if diff < min_diff {
			min_diff = diff;
			found = i as i32;
		}
	}
	
	let note = if found == -1 {
		println!("Failed to find note name");
		String::new()
	} else {
		let octave = found / 12;
		let name = ptmf::NOTE_NAMES[(found % 12) as usize];
		if octave == 1 {
			// Lower case
			name.to_lowercase()
		} else if octave == 2 {
			// Upper case
			name.to_uppercase()			
		} else {
			format!("O{}{}",octave,name)
		}
	};

	note
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

	if args.flag_format_jbe {
		// Write in which order to play patterns
		write!(writer, "song\n").unwrap();
		write!(writer, "{{\n\t").unwrap();
		for i in 0..module.length {
			let position = module.positions.data[i as usize];
			if i == 0 {
				write!(writer, "{}", position).unwrap();
			} else {
				write!(writer, ",{}", position).unwrap();
			}
		}		
		write!(writer, "\n}}\n").unwrap();

		// Write patterns
		write!(writer, "patterns\n").unwrap();
		write!(writer, "{{\n").unwrap();
		let mut pattern_no = 0;
		for pattern in module.patterns {
			write!(writer,"\t{}:", pattern_no).unwrap();
			// Loop through the pattern three times
			// 0 = just sample number
			// 1,2 = just pitch
			let mut row_no = 0;
			for row in &pattern.rows {
				let ref channel = row.channels[0];
				if row_no != 0 {
					write!(writer,",").unwrap();
				}
				if channel.sample_number > 0 {
					let number = channel.sample_number as i8;
					write!(writer,"S{}", number).unwrap();
				}
				row_no += 1;
			}
			write!(writer,"\n").unwrap();

			write!(writer,"\t{}:", pattern_no).unwrap();
			row_no = 0;
			for row in &pattern.rows {
				let ref channel = row.channels[1];
				if row_no != 0 {
					write!(writer,",").unwrap();
				}
				if channel.period > 0 {
					let note = note_from_period(channel.period);
					write!(writer,"{}", note).unwrap();
				}
				row_no += 1;
			}
			write!(writer,"\n").unwrap();

			write!(writer,"\t{}:", pattern_no).unwrap();
			row_no = 0;
			for row in &pattern.rows {
				let ref channel = row.channels[2];
				if row_no != 0 {
					write!(writer,",").unwrap();
				}
				if channel.period > 0 {
					let note = note_from_period(channel.period);
					write!(writer,"{}", note).unwrap();
				}
				row_no += 1;
			}
			write!(writer,"\n").unwrap();

			write!(writer,"\n").unwrap();
			pattern_no += 1;
		}		
		write!(writer, "}}\n").unwrap();
	} else {
		// Write in which order to play patterns
		write!(writer, "song_pattern_list:\n").unwrap();
		for i in 0..module.length {
			let position = module.positions.data[i as usize];
			write!(writer, "\tdefw song_pattern_{}\n", position).unwrap();
		}
		// Terminate list
		write!(writer, "\tdefw 0\n").unwrap();
		write!(writer, "\n").unwrap();
		
		// Write pattern data
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
