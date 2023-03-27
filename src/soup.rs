use std::fmt::Display;
use std::hash::Hash;

pub trait SoupType:
	Clone + Copy +
	PartialEq + Ord +
	Hash +
	Display
{}

#[derive(Clone,Debug)]
#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub struct Soup<T: SoupType> {
	pub states : Vec<T>
}
#[allow(dead_code)]
impl<T: SoupType> Soup<T> {
	pub fn new(states : Vec<T>) -> Self { Soup{ states } }
	pub fn certain(&self) -> Option<T> {
		if self.states.len() == 1 {
			return self.states.first().copied();
		}
		else { None }
	}
	pub fn impossible(&self) -> bool { return self.states.len()==0; }
}
