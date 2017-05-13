# mod2abc
Simple converter for Protracker modfiles to something that can be played on Luxor ABC80

Compile:
1. Install Rust and Cargo
https://www.rust-lang.org/en-US/
https://crates.io/
Cargo is usually shipped with Rust.
2. git clone https://github.com/hypp/mod2abc.git
3. cargo build --release

Usage:
1. Create a .mod file in Protracker.
Only the instrument numbers will be used.
2. Convert the .mod file using
mod2abc --in <filename> --out song.s
3. Edit instruments.s to your liking.
One instrument definition per instrument used in the .mod file
4. Create a program for Luxor ABC80 and include tracker.s
Tracker.s will include instruments.s and songs.
5. In the program, 
call tracker_init at initialization and
call tracker_play each vertical blanking
