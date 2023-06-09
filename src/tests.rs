#[cfg(test)]
mod tests {
	use crate::*;
	use templates::char::*;
	fn example_rules() -> Rules<char> {
		let sample = vec![
					" #     ",
					"###  # ",
					"#o## # ",
					"###    "];
		Rules::induce(vec![' ','#','o'], strings_to_vecs(&sample))
	}

	#[test]
	fn basic_induction() {
		let sample = vec![ " #     ", "###  # ", "#o## # ", "###    "];
		let rules = Rules::induce(vec![' ','#','o'], strings_to_vecs(&sample));
		rules.disallow.iter().for_each(|(k, v)|
			match k.0 {
				'o' => { assert!(v.contains(&' ')); assert!(v.contains(&'o')); }
				' ' => { assert!(v.contains(&'o')); assert!(v.len()==1); }
				_ => {}
			}
		);
	}

	#[test]
	fn deterministic() {
		let r = example_rules();
		let mut g1 = Grid::new(6, 6, r.clone());
		let mut g2 = Grid::new(6, 6, r.clone());
		let deterministic_order = |c : &Soup<char>| {
			c.states.clone()
				.iter()
				.map(|x| x.clone())
				.collect()
		};

		g1.collapse_certain((2,2), '#').expect("initial g1 collapse");
		g2.collapse_certain((2,2), '#').expect("initial g2 collapse");
		g1.bruteforce_collapse(&deterministic_order).expect("bruteforce collapse of g1");
		g2.bruteforce_collapse(&deterministic_order).expect("bruteforce collapse of g2");

		g1.print();
		g2.print();
		assert_eq!(g1, g2);
	}
}
