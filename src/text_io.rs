macro_rules! dbg {
	() => (eprintln!("[RUST]"));
	($fmt:expr) => (eprintln!(concat!("[RUST]	", $fmt)));
	($fmt:expr, $($arg:tt)*) => (eprintln!(concat!("[RUST]	", $fmt), $($arg)*));
}

use std::os::raw::c_char;
use std::ffi::CString;

use set::rf_Set;
use relation::rf_Relation;

#[no_mangle]
pub extern fn rf_set_to_string(s_ptr: *mut rf_Set) -> *const c_char {
	dbg!("rf_set_to_string({:?})", s_ptr);
	let s = unsafe { s_ptr.as_ref() }.unwrap();
	let c_string = CString::new(s.to_string()).unwrap();
	return CString::into_raw(c_string);
}

#[no_mangle]
pub extern fn rf_relation_to_string(r_ptr: *mut rf_Relation) -> *const c_char {
	dbg!("rf_relation_to_string({:?})", r_ptr);
	let r = unsafe { r_ptr.as_ref() }.unwrap();
	let c_string = CString::new(r.to_string()).unwrap();
	return CString::into_raw(c_string);
}

#[no_mangle]
pub extern fn rf_string_free(m_ptr: *mut c_char) {
	if m_ptr.is_null() { return }
	// m is dropped when it goes out of scope,
	// but let's be explicit here
	let m = unsafe { CString::from_raw(m_ptr) };
	drop(m);
}
