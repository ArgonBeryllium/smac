use std::collections::HashMap;
use crate::soup::*;
use crate::rules::Rules;
use crate::collapse_helpers::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T: SoupType> {
	w: u32,
	h: u32,
	rules : Rules<T>,
	cells: HashMap<(u32,u32), Soup<T>>
}
#[allow(dead_code)]
impl<T: SoupType> Grid<T> {
	pub fn new(w : u32, h : u32, rules : Rules<T>) -> Self {
		let mut cells = HashMap::new();
		for x in 0..w {
			for y in 0..h {
				cells.insert((x,y), Soup::new(rules.types.clone()));
			}
		}
		Grid{w, h, rules, cells}
	}
	pub fn get_width(&self) -> u32 { self.w }
	pub fn get_height(&self) -> u32 { self.h }
	pub fn get_rules(&self) -> &Rules<T> { &self.rules }
	pub fn get_rules_mut(&mut self) -> &mut Rules<T> { &mut self.rules }

	pub fn get(&self, c : (u32,u32)) -> Soup<T> {
		self.cells.get(&c).unwrap().clone()
	}
	pub fn set(&mut self, c : (u32, u32), s : Soup<T>) {
		self.cells.insert(c, s);
	}
	pub fn collapse(&mut self,
		pos: (u32,u32),
		states : Soup<T>) -> Result<(), CollapseError<T>>
	{
		if pos.0 >= self.w || pos.1 >= self.h {
			return Err(CollapseError::Other(pos,
					"Invalid position".to_owned()))
		}
		let mut history = CollapseHistory::new();
		history.push_bef(pos, [(pos, self.cells[&pos].clone())].into());
		self.cells.insert(pos, states.clone());
		history.push_aft([(pos, states)].into());

		self.propagate_collapse(pos, &mut history)
	}
	pub fn collapse_certain(&mut self,
		pos: (u32,u32),
		state : T) -> Result<(), CollapseError<T>>
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
	fn get_uncertain_cells(&self) -> HashMap<(u32, u32), Soup<T>> {
		self.cells.iter()
			.filter(|(_, v)| v.certain().is_none())
			.map(|(k, v)| (k.clone(), v.clone()))
			.collect::<HashMap<(u32,u32), Soup<T>>>()
	}

	fn propagate_collapse(&mut self,
		o: (u32,u32),
		hist : &mut CollapseHistory<T>) -> Result<(), CollapseError<T>>
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
		order : &dyn Fn(&Soup<T>) -> Vec<T>) -> Option<CollapseHistory<T>>
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
		hist : &mut CollapseHistory<T>,
		order : &dyn Fn(&Soup<T>) -> Vec<T>) -> bool
	{
		let uncertain_cells = self.get_uncertain_cells();
		if uncertain_cells.len() == 0 { return true; }

		let left = uncertain_cells.clone();
		let mut left : Vec<(&(u32, u32), &Soup<T>)> = left.iter().collect();
		// TODO make this optional, as determinism is not always necessary
		left.sort();

		while left.len() > 0 {
			let cell = left[0];
			let cell = (cell.0.clone(), cell.1.clone());
			let mut options = order(&cell.1);

			while options.len() > 0 {
				let option = options.pop().unwrap();

				hist.push_bef(cell.0, uncertain_cells.clone());
				let r = self.collapse_certain(cell.0, option);
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
						CollapseError::Other(pos, msg) =>
							panic!("Collapse error in bruteforce_iter at position {:?}: {msg})", pos)
					};
				}
			}
			left.remove(0);
		}

		false
	}
}
