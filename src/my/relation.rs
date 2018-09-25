use std::vec::Vec;
use std::fmt;

use my::set;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relation {
	domain: (Vec<set::SetElement>, Vec<set::SetElement>),
	table: Vec<bool>,
}

impl Relation {
	pub fn new(p: Vec<set::SetElement>, q: Vec<set::SetElement>, table: Vec<bool>) -> Relation {
		Relation {
			domain: (p, q),
			table: table,
		}
	}
}

impl fmt::Display for Relation {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[TODO] fmt::Display for Relation :)")
	}
}
