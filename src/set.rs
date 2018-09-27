macro_rules! dbg {
	() => (eprintln!("[RUST]"));
	($fmt:expr) => (eprintln!(concat!("[RUST]	", $fmt)));
	($fmt:expr, $($arg:tt)*) => (eprintln!(concat!("[RUST]	", $fmt), $($arg)*));
}

use relax;

use std::os::raw::{c_char,c_int};
use std::ffi::CStr;
use std::{ptr, slice};

#[allow(non_camel_case_types)]
pub type rf_Set = relax::Set;
#[allow(non_camel_case_types)]
pub type rf_SetElement = relax::SetElement;

#[no_mangle]
pub extern fn rf_set_element_new_string(c_buf: *mut c_char) -> *mut rf_SetElement {
	let c_str = unsafe { CStr::from_ptr(c_buf) };
	let r_str = c_str.to_str().unwrap(); // ok if c_buf/c_str is valid UTF-8
	// rf_SetElement not recognized for some reason
	let e = Box::new(relax::SetElement::Str(String::from(r_str)));
	return Box::into_raw(e);
}

#[no_mangle]
pub extern fn rf_set_element_new_set(s_ptr: *mut rf_Set) -> *mut rf_SetElement {
	let s = unsafe { s_ptr.as_ref() }.unwrap();
	// rf_SetElement not recognized for some reason
	let e = Box::new(relax::SetElement::Set(s.clone()));
	return Box::into_raw(e);
}

#[no_mangle]
pub extern fn rf_set_new(n: c_int, es_ptr: *mut *mut rf_SetElement) -> *mut rf_Set {
	let es: &[*mut rf_SetElement] = unsafe { slice::from_raw_parts(es_ptr, n as usize) };
	let tmp: rf_Set = es.into_iter()
		.map(|e_ptr| unsafe { *Box::from_raw(*e_ptr) })
		.collect();
	let s = Box::new(tmp);
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
pub extern fn rf_set_new_intersection(p_ptr: *mut rf_Set, q_ptr: *mut rf_Set) -> *mut rf_Set {
	let p = unsafe { p_ptr.as_ref() }.unwrap();
	let q = unsafe { q_ptr.as_ref() }.unwrap();
	let s = Box::new(rf_Set::intersection(&p, &q).cloned().collect());
	return Box::into_raw(s);
}

#[no_mangle]
pub extern fn rf_set_new_powerset(s_ptr: *mut rf_Set) -> *mut rf_Set {
	dbg!("set_powerset from rust");
	let s = Box::new(rf_Set::new()); // FIXME
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
