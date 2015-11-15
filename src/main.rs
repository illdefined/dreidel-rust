extern crate rand;

use rand::{Rng, OsRng};

fn main() {
	let letters = ['נ', 'ג', 'ה', 'ש'];
	let mut rng = match OsRng::new() {
		Ok(g) => g,
		Err(e) => panic!("Failed to create RNG: {}", e)
	};

	match rng.choose(&letters) {
		Some(l) => println!("{}", l),
		None => panic!("Could not generate random number")
	}
}
