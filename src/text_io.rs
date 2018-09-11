use std::os::raw::c_char;
use std::ffi::CString;

use rf_Set;
use rf_Relation;

#[no_mangle]
pub unsafe extern "C" fn rf_set_to_string(s: rf_Set) -> *const c_char {
	println!("set_to_string from rust");
	return CString::new("dummy").unwrap().as_ptr();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_to_string(r: rf_Relation) -> *const c_char {
	println!("relation_to_string from rust");
	return CString::new("dummy").unwrap().as_ptr();
}
