pub mod char {
	use crate::soup::SoupType;
	use crate::grid::Grid;
	pub fn strings_to_vecs(strings : &Vec<&str>) -> Vec<Vec<char>> {
		strings.iter()
			.map(|l|
				l.chars().collect::<Vec<char>>().clone())
			.collect()
	}
	pub fn get_unique_chars(strings : &Vec<&str>) -> Vec<char> {
		let mut out = Vec::new();
		for l in strings.iter() {
			for c in l.chars() {
				if !out.contains(&c) { out.push(c); }
			}
		}
		out
	}

	impl SoupType for char {}
	pub trait Printable { fn print(&self) {} }
	impl Printable for Grid<char> {
		fn print(&self) {
			println!("---");
			for y in 0..self.get_height() {
				for x in 0..self.get_width() {
					let c = self.get((x, y));
					let n = c.states.len();
					if n != 1 { print!("{n} ") }
					else {
						let c = c.certain();
						if c.is_some() { print!("{} ", c.unwrap()) }
						else { print!("? ") }
					}
				}
				println!();
			}
		}
	}
}
