#[derive(Clone)]
pub struct Soup {
	pub states : Vec<char>
}
#[allow(dead_code)]
impl Soup {
	pub fn new(chars : Vec<char>) -> Self { Soup{ states: chars.clone() } }
	pub fn certain(&self) -> Option<char> {
		if self.states.len() == 1 {
			return self.states.first().copied();
		}
		else { None }
	}
	pub fn impossible(&self) -> bool { return self.states.len()==0; }
}
