macro_rules! dbg {
	() => (eprintln!("[RUST]"));
	($fmt:expr) => (eprintln!(concat!("[RUST]	", $fmt)));
	($fmt:expr, $($arg:tt)*) => (eprintln!(concat!("[RUST]	", $fmt), $($arg)*));
}

use std::collections::BTreeSet;
use std::fmt;

use std::os::raw::{c_char,c_int};
use std::ffi::CStr;
use std::ptr;
use std::slice;

/* HashSet does not implement Hash, so it cannot be nested (currently) */
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct rf_Set(BTreeSet<rf_SetElement>);

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum rf_SetElement {
	Str(String),
	Set(rf_Set),
}

impl rf_Set {
	pub fn cardinality(&self) -> usize {
		self.0.len()
	}
	pub fn iter(&self) -> ::std::collections::btree_set::Iter<rf_SetElement> {
		self.0.iter()
	}
	pub fn is_subset(&self, other: &rf_Set) -> bool {
		self.0.is_subset(&other.0)
	}
	pub fn intersection<'a>(&'a self, other: &'a rf_Set) -> ::std::collections::btree_set::Intersection<'a, rf_SetElement> {
		self.0.intersection(&other.0)
	}
}

impl fmt::Display for rf_Set {
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

impl fmt::Display for rf_SetElement {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			rf_SetElement::Str(s) => write!(f, "{}", s),
			rf_SetElement::Set(s) => write!(f, "{}", s),
		}
	}
}

#[no_mangle]
pub extern fn rf_set_element_new_string(c_buf: *mut c_char) -> *mut rf_SetElement {
	let c_str = unsafe { CStr::from_ptr(c_buf) };
	let r_str = c_str.to_str().unwrap(); // ok if c_buf/c_str is valid UTF-8
	let e = Box::new(rf_SetElement::Str(String::from(r_str)));
	return Box::into_raw(e);
}

#[no_mangle]
pub extern fn rf_set_element_new_set(s_ptr: *mut rf_Set) -> *mut rf_SetElement {
	let s = unsafe { s_ptr.as_ref() }.unwrap();
	let e = Box::new(rf_SetElement::Set(s.clone()));
	return Box::into_raw(e);
}

#[no_mangle]
pub extern fn rf_set_new(n: c_int, es_ptr: *mut *mut rf_SetElement) -> *mut rf_Set {
	let es: &[*mut rf_SetElement] = unsafe { slice::from_raw_parts(es_ptr, n as usize) };
	let collection: BTreeSet<_> = es.into_iter()
		.map(|e_ptr| unsafe { *Box::from_raw(*e_ptr) })
		.collect();
	let s = Box::new(rf_Set(collection));
	dbg!("set_new(...): {:?}", s);
	return Box::into_raw(s);
}

#[no_mangle]
pub extern fn rf_set_clone(s_ptr: *mut rf_Set) -> *mut rf_Set {
	if s_ptr.is_null() {
		return ptr::null_mut();
	}
	let s = unsafe { s_ptr.as_ref() }.unwrap();
	let cpy = Box::new((*s).clone());
	let cpy_ptr = Box::into_raw(cpy);
	dbg!("set_clone({:?}): {:?}", s_ptr, cpy_ptr);
	return cpy_ptr;
}

#[no_mangle]
pub extern "C" fn rf_set_new_intersection(p_ptr: *mut rf_Set, q_ptr: *mut rf_Set) -> *mut rf_Set {
	dbg!("set_intersection from rust");
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let s = Box::new(rf_Set(rf_Set::intersection(&p, &q).cloned().collect()));
	return Box::into_raw(s);
}

#[no_mangle]
pub extern "C" fn rf_set_new_powerset(s_ptr: *mut rf_Set) -> *mut rf_Set {
	dbg!("set_powerset from rust");
	let s = Box::new(rf_Set(BTreeSet::new())); // FIXME
	return Box::into_raw(s);
}

#[no_mangle]
pub extern fn rf_set_equal(p_ptr: *mut rf_Set, q_ptr: *mut rf_Set) -> bool {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	return p == q;
}

#[no_mangle]
pub extern fn rf_set_is_subset(p_ptr: *mut rf_Set, q_ptr: *mut rf_Set) -> bool {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	return p.is_subset(&q);
}

#[no_mangle]
pub extern fn rf_set_get_cardinality(s_ptr: *mut rf_Set) -> c_int {
	let s = unsafe { s_ptr.as_ref() }.unwrap();
	return s.cardinality() as c_int;
}

#[no_mangle]
pub extern fn rf_set_free(m_ptr: *mut rf_Set) {
	dbg!("set_free({:?})", m_ptr);
	if m_ptr.is_null() { return; }
	// m is dropped when it goes out of scope,
	// but let's be explicit here
	let m = unsafe { Box::from_raw(m_ptr) };
	drop(m);
}
