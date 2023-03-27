use rand::seq::SliceRandom;

mod tests;
mod soup;
mod rules;
mod collapse_helpers;
mod grid;
use soup::{Soup, SoupType};
use rules::Rules;
use grid::Grid;

impl SoupType for char {
}

fn strings_to_vecs(strings : &Vec<&str>) -> Vec<Vec<char>> {
	strings.iter()
		.map(|l|
			l.chars().collect::<Vec<char>>().clone())
		.collect()
}
trait Printable { fn print(&self) {} }
impl Printable for Grid<char> {
	fn print(&self) {
		for y in 0..self.get_height() {
			for x in 0..self.get_width() {
				let c = self.get((x, y));
				let n = c.states.len();
				if n != 1 { print!("{n} ") }
				else if let c = c.certain() {
					if c.is_some() { print!("{} ", c.unwrap()) }
					else { print!("? ") }
				}
			}
			println!();
		}
	}
}

fn main() {
	let sample = vec![
		"   |  || ",
		" r-+--++-",
		" | |  ||r",
		" | |  ||L",
		" L-+--++-"
	];
	let states = vec![' ', '-', '|', '+', 'r', 'L'];
	let rules = Rules::induce(states.clone(), strings_to_vecs(&sample));
	println!("{:?}", rules.disallow);

	let mut grid : Grid<char> = Grid::new(5,5, rules);
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
