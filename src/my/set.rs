use std::collections::BTreeSet;
use std::fmt;
use std::iter;

/* HashSet does not implement Hash, so it cannot be nested (currently) */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set(BTreeSet<SetElement>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetElement {
	Str(String),
	Set(Set),
}

impl Set {
	pub fn new() -> Set {
		Set(BTreeSet::new())
	}
	pub fn cardinality(&self) -> usize {
		self.0.len()
	}
	pub fn iter(&self) -> ::std::collections::btree_set::Iter<SetElement> {
		self.0.iter()
	}
	pub fn is_subset(&self, other: &Set) -> bool {
		self.0.is_subset(&other.0)
	}
	pub fn intersection<'a>(&'a self, other: &'a Set) -> ::std::collections::btree_set::Intersection<'a, SetElement> {
		self.0.intersection(&other.0)
	}
}

impl fmt::Display for Set {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{{")?;
		let mut it = self.0.iter();
		if let Some(e) = it.next() {
			write!(f, "{}", e)?;
			for e in it {
				write!(f, " {}", e)?;
			}
		}
		write!(f, "}}")
	}
}

impl iter::FromIterator<SetElement> for Set {
	fn from_iter<I: IntoIterator<Item = SetElement>>(iter: I) -> Set {
		let mut s = Set::new();
		s.0.extend(iter);
		return s;
	}
}

impl fmt::Display for SetElement {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			SetElement::Str(s) => write!(f, "{}", s),
			SetElement::Set(s) => write!(f, "{}", s),
		}
	}
}
