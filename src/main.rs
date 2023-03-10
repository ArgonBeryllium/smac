use std::{collections::HashMap, fmt::Debug};

use rand::seq::SliceRandom;

#[derive(Clone)]
struct Soup {
	states : Vec<char>
}
#[allow(dead_code)]
impl Soup {
	fn new(chars : Vec<char>) -> Self { Soup{ states: chars.clone() } }
	fn certain(&self) -> Option<char> {
		if self.states.len() == 1 {
			return self.states.first().copied();
		}
		else { None }
	}
	fn impossible(&self) -> bool { return self.states.len()==0; }
}
struct Rules {
	chars : Vec<char>,
	disallow : HashMap<(char, (i32, i32)), Vec<char>>
}
#[allow(dead_code)]
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
				if ox == 0 && oy == 0 { continue; }
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

	fn update_cell(&self,
		c : &mut Soup,
		nwo : Vec<(&Soup, (i32, i32))>) -> usize {
		let mut to_remove = Vec::new();
		// TODO make this more involved;
		// current approach likely overlooks some configurations
		for (n, o) in nwo {
			for state in c.states.iter() {
				let m = n.certain();
				if m.is_none() { continue; }
				if self.disallow[&(state.clone(), o)].contains(&m.unwrap()) {
					to_remove.push(state.clone());
				}
			}
		}
		c.states.retain(|e| !to_remove.contains(e));
		c.states.len()
	}
}

#[derive(Debug)]
#[allow(dead_code)]
enum CollapseError {
	Impossible((u32,u32), CollapseHistory),
	Other((u32,u32),String)
}

#[derive(Clone)]
#[allow(dead_code)]
struct GridChange {
	bef : HashMap<(u32,u32), Soup>,
	aft : HashMap<(u32,u32), Soup>
}
#[derive(Clone)]
struct CollapseHistory {
	changes : Vec<((u32, u32), GridChange)>,
	open : bool
}
impl CollapseHistory {
	fn new() -> Self { Self { changes: Vec::new(), open : false } }

	fn push_bef(&mut self,
		pos : (u32, u32),
		bef : HashMap<(u32, u32), Soup>) {
		assert!(!self.open, "Pushing `bef` to history with an open entry");
		self.open = true;
		self.changes.push((pos, GridChange{bef, aft: HashMap::new()}));
	}
	fn push_aft(&mut self, aft : HashMap<(u32, u32), Soup>) {
		assert!(self.open, "Pushing `aft` to history without an open entry");
		self.changes.last_mut().unwrap().1.aft = aft;
		self.open = false;
	}

	fn contains(&self, pos : &(u32, u32)) -> bool
		{ self.changes.iter().any(|e| e.0 == *pos) }

	fn undo(&mut self,
		grid : &mut Grid,
		steps : usize) -> Vec<((u32, u32), GridChange)>
	{
		let range =
			if steps == 0 { 0..self.changes.len() }
			else { 0..steps };

		let mut out = Vec::new();
		for _ in range {
			let change = self.changes.pop().expect("No changes left to undo");
			out.push(change.clone());
			for cell_state in change.1.bef {
				grid.set(cell_state.0, cell_state.1);
			}
		}
		out
	}
}
impl Debug for CollapseHistory{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("Collapse History")
			.field("total entries", &self.changes.len())
			.field("first change pos", &self.changes.first().unwrap().0)
			.field("last change pos", &self.changes.last().unwrap().0)
			.finish()
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

	pub fn get(&self, c : (u32,u32)) -> Soup {
		self.cells.get(&c).unwrap().clone()
	}
	pub fn set(&mut self, c : (u32, u32), s : Soup) {
		self.cells.insert(c, s);
	}
	pub fn collapse(&mut self,
		pos: (u32,u32),
		states : Soup) -> Result<(), CollapseError>
	{
		self.cells.insert(pos, states);
		self.propagate_collapse(pos, &mut CollapseHistory::new())
	}
	pub fn collapse_certain(&mut self,
		pos: (u32,u32),
		state : char) -> Result<(), CollapseError>
	{
		self.collapse(pos, Soup::new(vec![state]))
	}

	fn get_neighbours_with_offsets(&self,
		x : u32,
		y : u32) -> Vec<((u32, u32), (i32, i32))>
	{
		let mut out = Vec::new();
		// for slightly less annoying comparisons
		let x = x as i32;
		let y = y as i32;

		for ox in -1..=1 {
			let rx = x + ox;
			if rx < 0 || rx >= (self.w as i32) { continue; }
			for oy in -1..=1 {
				if ox == 0 && oy == 0  { continue; }
				let ry = y + oy;
				if ry < 0 || ry >= (self.h as i32) { continue; }
				out.push(((rx as u32, ry as u32), (ox, oy)));
			}
		}
		out
	}
	fn get_uncertain(&self) -> HashMap<(u32, u32), Soup> {
		self.cells.iter()
			.filter(|(_, v)| v.certain().is_none())
			.map(|(k, v)| (k.clone(), v.clone()))
			.collect::<HashMap<(u32,u32), Soup>>()
	}
	fn print(&self) {
		println!("---");
		for y in 0..self.h {
			for x in 0..self.w {
				let c = self.cells.get(&(x,y)).unwrap();
				let c = c.certain()
					.unwrap_or(c.states.len()
						.to_string().chars().nth(0).unwrap());
				print!("{} ", c);
			}
			println!();
		}
	}

	fn propagate_collapse(&mut self,
		o: (u32,u32),
		hist : &mut CollapseHistory) -> Result<(), CollapseError>
	{
		let nbrs_and_offsets = self.get_neighbours_with_offsets(o.0, o.1);

		let nbr_values = nbrs_and_offsets.iter()
			.filter(|(p,_)| self.cells.contains_key(p))
			.map(|(n, _)| (n.clone(), self.get(*n).clone()))
			.into_iter().collect();
		hist.push_bef(o, nbr_values);

		// apply the consequences to neighbours
		let o_value = self.get(o).clone();
		for (nbr, nbr_offset) in nbrs_and_offsets.iter() {
			if hist.contains(nbr) { continue; }

			let r = self.rules.update_cell(
				self.cells.get_mut(nbr).unwrap(),
				vec![(&o_value, (-nbr_offset.0, -nbr_offset.1))]
			);
			if r==0 {
				return Err(CollapseError::Impossible(
					nbr.clone(),
					hist.clone()
				));
			}
		}

		let nbr_values = nbrs_and_offsets.iter()
			.filter(|(p,_)| self.cells.contains_key(p))
			.map(|(n, _)| (n.clone(), self.get(*n).clone()))
			.into_iter().collect();
		hist.push_aft(nbr_values);

		// propagate until all have been updated
		for (nbr,_) in nbrs_and_offsets.iter() {
			if hist.contains(nbr) { continue; }
			let r = self.propagate_collapse(*nbr, hist);
			if r.is_err() { return r; }
		}

		Ok(())
	}

	pub fn bruteforce_collapse(&mut self,
		order : &dyn Fn(&Soup) -> Vec<char>) -> Option<CollapseHistory>
	{
		let mut hist = CollapseHistory::new();
		if self.bruteforce_iter(&mut hist, order) {
			Some(hist)
		}
		else {
			hist.undo(self, 0);
			None
		}
	}
	fn bruteforce_iter(&mut self,
		hist : &mut CollapseHistory,
		order : &dyn Fn(&Soup) -> Vec<char>) -> bool
	{
		let uncertain_cells = self.get_uncertain();
		if uncertain_cells.len() == 0 { return true; }

		let c = uncertain_cells.iter().nth(0).unwrap();
		let c = (c.0.clone(), c.1.clone());
		let mut options = order(&c.1);

		while options.len() > 0 {
			hist.push_bef(c.0, uncertain_cells.clone());
			let option = options.pop().unwrap();
			let r = self.collapse_certain(c.0, option);
			hist.push_aft(uncertain_cells.iter()
				.map(|(&p,_)| (p, self.get(p)))
				.collect());

			if r.is_ok() {
				if self.bruteforce_iter(hist, order) {
					return true;
				}
				else { hist.undo(self, 1); }
			}
			else {
				match r.err().unwrap() {
					CollapseError::Impossible(_, mut subhist) =>
						subhist.undo(self, 0),
					_ => todo!()
				};
			}
		}
		false
	}
}

fn main() {
	test();
	let sample = vec![
				" #     ",
				"###  # ",
				"#o## # ",
				"###    "];
	let rules = Rules::induce(vec![' ','#','o'], sample);
	println!("{:?}", rules.disallow);

	let mut grid = Grid::new(10,10, rules);
	grid.print();
	grid.collapse_certain((4, 5), 'o').expect("First collapse");
	grid.print();
	let order = |c : &Soup| {
		let mut out = c.states.clone();
		out.shuffle(&mut rand::thread_rng());
		out
	};
	grid.bruteforce_collapse(&order);
	grid.print();
}
fn test() {
	let sample = vec![
				" #     ",
				"###  # ",
				"#o## # ",
				"###    "];
	let rules = Rules::induce(vec![' ','#','o'], sample);
	rules.disallow.iter().for_each(|(k, v)|
		match k.0 {
			'o' => {
				assert!(v.contains(&' '));
				assert!(v.contains(&'o'));
			}
			' ' => {
				assert!(v.contains(&'o'));
				assert!(v.len()==1);
			}
			_ => {}
		}
	);
}
