use rand::seq::SliceRandom;
use smac::prelude::*;

fn main() {
	let sample = vec![
		"   |  || ",
		" r-+--++-",
		" | |  ||r",
		" | |  ||L",
		" L-+--++-",
		"   |  || "
	];
	let states = templates::char::get_unique_chars(&sample);
	let rules = Rules::induce(states.clone(), templates::char::strings_to_vecs(&sample));

	let mut grid : Grid<char> = Grid::new(10,10, rules);
	grid.print();
	grid.collapse_certain((2, 3), states[2]).expect("First collapse");
	grid.print();
	let order = |c : &Soup<char>| {
		let mut out = c.states.clone();
		out.shuffle(&mut rand::thread_rng());
		out
	};
	println!("{}", grid.bruteforce_collapse(&order).is_some());
	grid.print();
}
