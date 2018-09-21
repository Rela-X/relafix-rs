macro_rules! dbg {
	() => (eprintln!("[RUST]"));
	($fmt:expr) => (eprintln!(concat!("[RUST]	", $fmt)));
	($fmt:expr, $($arg:tt)*) => (eprintln!(concat!("[RUST]	", $fmt), $($arg)*));
}

use std::vec::Vec;
use std::fmt;

use std::slice;
use std::ptr;

use set;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyRel {
	domain: (Vec<set::rf_SetElement>, Vec<set::rf_SetElement>),
	table: Vec<bool>,
}

impl MyRel {
}

impl fmt::Display for MyRel {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[TODO] fmt::Display for Relation :)")
	}
}

#[allow(non_camel_case_types)]
pub type rf_Relation = MyRel; //relax::relation::relation_vec::RelationVec<'a, P, Q>;

#[no_mangle]
pub extern fn rf_relation_new(p_ptr: *mut set::rf_Set, q_ptr: *mut set::rf_Set, table_ptr: *mut bool) -> *mut rf_Relation {
	dbg!("relation_new({:?}, {:?})", p_ptr, q_ptr);
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let n = p.cardinality() * q.cardinality();
	let t: &[bool] = unsafe { slice::from_raw_parts(table_ptr, n as usize) };
	let r = Box::new(
		MyRel {
			domain: (p.iter().cloned().collect(), q.iter().cloned().collect()),
			table: t.into(),
		}
	);
	return Box::into_raw(r);
}

#[no_mangle]
pub extern fn rf_relation_clone(r_ptr: *mut rf_Relation) -> *mut rf_Relation {
	if r_ptr.is_null() {
		return ptr::null_mut();
	}
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	let cpy = Box::new((*r).clone());
	let cpy_ptr = Box::into_raw(cpy);
	dbg!("relation_clone({:?}): {:?}", r_ptr, cpy_ptr);
	return cpy_ptr;
}

#[no_mangle]
pub extern fn rf_relation_new_union(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> () {
	println!("relation_new_union from rust");
}

#[no_mangle]
pub extern fn rf_relation_new_intersection(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> () {
	println!("relation_new_intersection from rust");
}

#[no_mangle]
pub extern fn rf_relation_new_concatenation(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> () {
	println!("relation_new_concatenation from rust");
}

#[no_mangle]
pub extern fn rf_relation_new_complement(r_ptr: *mut rf_Relation) -> () {
	println!("relation_new_complement from rust");
}

#[no_mangle]
pub extern fn rf_relation_new_converse(r_ptr: *mut rf_Relation) -> () {
	println!("relation_new_converse from rust");
}

#[no_mangle]
pub extern fn rf_relation_is_homogeneous(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_homogeneous from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_reflexive(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_reflexive from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_irreflexive(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_irreflexive from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_antisymmetric(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_antisymmetric from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_transitive(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_transitive from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_symmetric(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_symmetric from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_asymmetric(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_asymmetric from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_preorder(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_preorder from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_partial_order(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_partial_order from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_equivalent(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_equivalent from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_difunctional(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_difunctional from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_lattice(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_lattice from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_sublattice(r_ptr: *mut rf_Relation, s_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_sublattice from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_injective(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_injective from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_functional(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_functional from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_surjective(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_surjective from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_lefttotal(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_lefttotal from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_bijective(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_bijective from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_is_function(r_ptr: *mut rf_Relation) -> bool {
	println!("relation_is_function from rust");
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return true;
}

#[no_mangle]
pub extern fn rf_relation_free(m_ptr: *mut rf_Relation) {
	dbg!("relation_free({:?})", m_ptr);
	if m_ptr.is_null() { return }
	// m is dropped when it goes out of scope,
	// but let's be explicit here
	let m = unsafe { Box::from_raw(m_ptr) };
	drop(m);
}
