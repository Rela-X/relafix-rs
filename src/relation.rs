use std::{ptr, slice};

use set;

use relax;
use relax::{Relation, Endorelation};

#[allow(non_camel_case_types)]
pub type rf_Relation = relax::RelationVec;

#[no_mangle]
pub extern fn rf_relation_new(p_ptr: *mut set::rf_Set, q_ptr: *mut set::rf_Set, table_ptr: *mut bool) -> *mut rf_Relation {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let n = p.cardinality() * q.cardinality();
	let t: &[bool] = unsafe { slice::from_raw_parts(table_ptr, n as usize) };
	let r = rf_Relation::new(
		(p.iter().cloned().collect(), q.iter().cloned().collect()),
		t.into(),
	);
	return Box::into_raw(Box::new(r));
}

#[no_mangle]
pub extern fn rf_relation_clone(r_ptr: *mut rf_Relation) -> *mut rf_Relation {
	if r_ptr.is_null() {
		return ptr::null_mut();
	}
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	let cpy = (*r).clone();
	let cpy_ptr = Box::into_raw(Box::new(cpy));
	return cpy_ptr;
}

#[no_mangle]
pub extern fn rf_relation_new_union(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> *mut rf_Relation {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let u = rf_Relation::union(p, q);
	let r = rf_Relation::from_relation(&u);
	return Box::into_raw(Box::new(r));
}

#[no_mangle]
pub extern fn rf_relation_new_intersection(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> *mut rf_Relation {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let i = rf_Relation::intersection(p, q);
	let r = rf_Relation::from_relation(&i);
	return Box::into_raw(Box::new(r));
}

#[no_mangle]
pub extern fn rf_relation_new_concatenation(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> *mut rf_Relation {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let c = rf_Relation::concatenation(p, q);
	let rc = rf_Relation::from_relation(&c);
	return Box::into_raw(Box::new(rc));
}

#[no_mangle]
pub extern fn rf_relation_new_complement(r_ptr: *mut rf_Relation) -> *mut rf_Relation {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	let c = rf_Relation::complement(r);
	let rc = rf_Relation::from_relation(&c);
	return Box::into_raw(Box::new(rc));
}

#[no_mangle]
pub extern fn rf_relation_new_converse(r_ptr: *mut rf_Relation) -> *mut rf_Relation {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	let c = rf_Relation::converse(r);
	let r = rf_Relation::from_relation(&c);
	return Box::into_raw(Box::new(r));
}

#[no_mangle]
pub extern fn rf_relation_equal(p_ptr: *mut rf_Relation, q_ptr: *mut rf_Relation) -> bool {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	return p == q;
}

#[no_mangle]
pub extern fn rf_relation_is_homogeneous(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_homogeneous();
}

#[no_mangle]
pub extern fn rf_relation_is_reflexive(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_reflexive();
}

#[no_mangle]
pub extern fn rf_relation_is_irreflexive(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_irreflexive();
}

#[no_mangle]
pub extern fn rf_relation_is_antisymmetric(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_antisymmetric();
}

#[no_mangle]
pub extern fn rf_relation_is_transitive(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_transitive();
}

#[no_mangle]
pub extern fn rf_relation_is_symmetric(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_symmetric();
}

#[no_mangle]
pub extern fn rf_relation_is_asymmetric(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_asymmetric();
}

#[no_mangle]
pub extern fn rf_relation_is_preorder(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_preorder();
}

#[no_mangle]
pub extern fn rf_relation_is_partial_order(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_partial_order();
}

#[no_mangle]
pub extern fn rf_relation_is_equivalent(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_equivalent();
}

#[no_mangle]
pub extern fn rf_relation_is_difunctional(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_difunctional();
}

#[no_mangle]
pub extern fn rf_relation_is_lattice(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_lattice();
}

#[no_mangle]
pub extern fn rf_relation_is_sublattice(sub_ptr: *mut rf_Relation, sup_ptr: *mut rf_Relation) -> bool {
	let sub = unsafe { sub_ptr.as_ref() }.unwrap();
	let sup = unsafe { sup_ptr.as_ref() }.unwrap();
	return sub.is_sublattice(sup);
}

#[no_mangle]
pub extern fn rf_relation_is_injective(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_injective();
}

#[no_mangle]
pub extern fn rf_relation_is_functional(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_functional();
}

#[no_mangle]
pub extern fn rf_relation_is_surjective(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_surjective();
}

#[no_mangle]
pub extern fn rf_relation_is_lefttotal(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_lefttotal();
}

#[no_mangle]
pub extern fn rf_relation_is_bijective(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_bijective();
}

#[no_mangle]
pub extern fn rf_relation_is_function(r_ptr: *mut rf_Relation) -> bool {
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	return r.is_function();
}

#[no_mangle]
pub extern fn rf_relation_free(m_ptr: *mut rf_Relation) {
	if m_ptr.is_null() { return }
	// m is dropped when it goes out of scope,
	// but let's be explicit here
	let m = unsafe { Box::from_raw(m_ptr) };
	drop(m);
}
