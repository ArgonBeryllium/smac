use core::fmt::Debug;
use std::collections::HashMap;

use crate::Grid;
use crate::Soup;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CollapseError {
	Impossible((u32,u32), CollapseHistory),
	Other((u32,u32),String)
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct GridChange {
	bef : HashMap<(u32,u32), Soup>,
	aft : HashMap<(u32,u32), Soup>
}
#[derive(Clone)]
pub struct CollapseHistory {
	changes : Vec<((u32, u32), GridChange)>,
	open : bool
}
impl CollapseHistory {
	pub fn new() -> Self { Self { changes: Vec::new(), open : false } }

	pub fn push_bef(&mut self,
		pos : (u32, u32),
		bef : HashMap<(u32, u32), Soup>) {
		assert!(!self.open, "Pushing `bef` to history with an open entry");
		self.open = true;
		self.changes.push((pos, GridChange{bef, aft: HashMap::new()}));
	}
	pub fn push_aft(&mut self, aft : HashMap<(u32, u32), Soup>) {
		assert!(self.open, "Pushing `aft` to history without an open entry");
		self.changes.last_mut().unwrap().1.aft = aft;
		self.open = false;
	}

	pub fn contains(&self, pos : &(u32, u32)) -> bool
		{ self.changes.iter().any(|e| e.0 == *pos) }

	pub fn undo(&mut self,
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

