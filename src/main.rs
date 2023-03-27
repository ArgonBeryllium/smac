use rand::seq::SliceRandom;

mod tests;
mod soup;
mod rules;
mod collapse_helpers;
mod grid;
use soup::Soup;
use rules::Rules;
use grid::Grid;

fn main() {
	let sample = vec![
		"   |  || ",
		" r-+--++-",
		" | |  ||r",
		" | |  ||L",
		" L-+--++-"
	];
	let states = vec![' ', '-', '|', '+', 'r', 'L'];
	let rules = Rules::induce(states.clone(), sample);
	println!("{:?}", rules.disallow);

	let mut grid = Grid::new(5,5, rules);
	grid.print();
	grid.collapse_certain((2, 3), states[2]).expect("First collapse");
	grid.print();
	let order = |c : &Soup| {
		let mut out = c.states.clone();
		out.shuffle(&mut rand::thread_rng());
		out
	};
	println!("{}", grid.bruteforce_collapse(&order).is_some());
	grid.print();
}
