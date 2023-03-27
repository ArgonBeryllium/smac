use rand::seq::SliceRandom;

mod tests;
mod soup;
mod rules;
mod collapse_helpers;
mod grid;
mod templates;
use soup::Soup;
use rules::Rules;
use grid::Grid;
use templates::char::*;

fn main() {
	let sample = vec![
		"   |  || ",
		" r-+--++-",
		" | |  ||r",
		" | |  ||L",
		" L-+--++-",
		"   |  || "
	];
	let states = vec![' ', '-', '|', '+', 'r', 'L'];
	let rules = Rules::induce(states.clone(), templates::char::strings_to_vecs(&sample));
	println!("{:?}", rules.disallow);

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
