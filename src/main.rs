use std::collections::HashMap;

#[derive(Clone)]
struct Soup {
	states : Vec<char>
}
impl Soup {
	fn new(chars : Vec<char>) -> Self { Soup{ states: chars.clone() } }
	fn certain(&self) -> Option<char> {
		if self.states.len() == 1 {
			return self.states.first().copied();
		}
		else { None }
	}
}
struct Rules {
	chars : Vec<char>,
	disallow : HashMap<(char, (i32, i32)), Vec<char>>
}
impl Rules {
	fn new() -> Self { Rules { chars: Vec::new(), disallow: HashMap::new() } }
	// induce the allowed neighbouring rules based on an example
	fn induce(chars : Vec<char>, example : Vec<&str>) -> Rules {
		let mut disallow = HashMap::new();
		let w = example[0].len();
		let h = example.len();
		example.iter().for_each(|e|
			assert!(e.len()==w, "Consistent width in example"));
		
		// yeah, I hate the nesting too
		for x in 0..w {
		for y in 0..h {
			for ox in -1i32..=1 {
			for oy in -1i32..=1 {
				let k = (example[y].chars().nth(x).unwrap(), (ox, oy));
				// get check offset validity
				let ry = y as i32 + oy;
				let rx = x as i32 + ox;
				if ry < 0 || rx < 0
					|| ry >= (h as i32)
					|| rx >= (w as i32) { continue; }

				// get neighbour value
				let d = example.get(ry as usize)
					.unwrap_or(&"")
					.chars().nth(rx as usize).unwrap();

				// if no rules exist yet for k, create them
				if !disallow.contains_key(&k) {
					disallow.insert(k, chars.clone());
				}
				// allow possibility
				let di = disallow[&k].iter().position(|&e| e==d);
				if di.is_some() {
					disallow.get_mut(&k).unwrap().remove(di.unwrap());
				}
			}}
		}}
		Rules { chars, disallow }
	}

	fn update_cell(&self, c : &mut Soup, nwo : Vec<(&Soup, (i32, i32))>) {
		let mut to_remove = Vec::new();
		for (n, o) in nwo {
			for state in c.states.iter() {
				// TODO finish this, only handles definite cases as of now
				let n = n.certain();
				if n.is_none() { continue; }
				let n = n.unwrap();
				if self.disallow[&(state.clone(), o)].contains(&n) {
					to_remove.push(state.clone());
					continue;
				}
			}
		}
		c.states.retain(|e| to_remove.contains(e));
	}
}
struct Grid {
	w: u32,
	h: u32,
	rules : Rules,
	cells: HashMap<(u32,u32), Soup>
}
impl Grid {
	pub fn new(w : u32, h : u32, rules : Rules) -> Self {
		let mut cells = HashMap::new();
		for x in 0..w {
			for y in 0..h {
				cells.insert((x,y), Soup::new(rules.chars.clone()));
			}
		}
		Grid{w, h, rules, cells}
	}

	fn print(&self) {
		println!("---");
		for y in 0..self.h {
			for x in 0..self.w {
				let c = self.cells.get(&(x,y)).unwrap()
					.certain().unwrap_or('?');
				//print!("{}", self.cells.get(&(x,y)).unwrap().states.len());
				print!("{} ", c);
			}
			println!();
		}
	}

	pub fn get_neighbours_with_offsets(&self, x : u32, y : u32)
		-> Vec<((u32, u32), (i32, i32))>
	{
		let mut out = Vec::new();
		// for slightly less annoying comparisons
		let x = x as i32;
		let y = y as i32;
		for ox in -1..=1 {
			let rx = x + ox;
			if rx < 0 || rx >= (self.w as i32) { continue; }
			for oy in -1..=1 {
				let ry = y + oy;
				if ry < 0 || ry >= (self.h as i32) { continue; }
				out.push(((rx as u32, ry as u32), (ox, oy)));
			}
		}
		out
	}
	pub fn collapse(&mut self, pos: (u32,u32), states : Soup) {
		self.cells.insert(pos, states);
		self.propagate_collapse(pos, &mut Vec::new());
	}
	pub fn collapse_certain(&mut self, pos: (u32,u32), state : char) {
		self.collapse(pos, Soup::new(vec![state]))
	}

	fn propagate_collapse(&mut self, o: (u32,u32), hist : &mut Vec<(u32, u32)>) {
		let mut s = self.get_neighbours_with_offsets(o.0, o.1);
		s.push((o, (0,0)));
		hist.push(o);

		let o_v = self.cells[&o].clone();
		for (e, e_o) in s.iter() {
			if hist.contains(e) { continue };
			hist.push(e.clone());
			self.rules.update_cell(self.cells.get_mut(e).unwrap(),
				vec![(&o_v.clone(), (-e_o.0, -e_o.1))]);
		}
		for (e,_) in s.iter() {
			if hist.contains(e) { continue };
			self.propagate_collapse(*e, hist);
		}		
	}
}

fn main() {
	let sample = vec![
				" #     ",
				"###  # ",
				"#o## # ",
				"###    "];
	let rules = Rules::induce(vec![' ','#','o'], sample);
	println!("{:?}", rules.disallow);
	let mut grid = Grid::new(10,10, rules);
	grid.print();
	grid.collapse_certain((4, 5), 'o');
	grid.print();
}