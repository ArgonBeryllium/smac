use crate::soup::Soup;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Rules {
	pub chars : Vec<char>,
	pub disallow : HashMap<(char, (i32, i32)), Vec<char>>
}
#[allow(dead_code)]
impl Rules {
	pub fn new() -> Self { Rules { chars: Vec::new(), disallow: HashMap::new() } }
	// induce the allowed neighbouring rules based on an example
	pub fn induce(chars : Vec<char>, example : Vec<&str>) -> Rules {
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

	pub fn update_cell(&self,
		c : &mut Soup,
		nwo : Vec<(&Soup, (i32, i32))>) -> usize {
		let mut to_remove = Vec::new();
		// TODO make this more involved;
		// current approach likely overlooks some configurations
		for (n, o) in nwo {
			for state in c.states.iter() {
				let m = n.certain();
				if m.is_none() { continue; }

				let disallowed = self.disallow.get(&(state.clone(), o));
				if disallowed.is_none() { continue; }
				if disallowed.unwrap().contains(&m.unwrap()) {
					to_remove.push(state.clone());
				}
			}
		}
		c.states.retain(|e| !to_remove.contains(e));
		c.states.len()
	}
}
