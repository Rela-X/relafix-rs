use std::os::raw::c_double;

use crate::set::rf_Set;
use crate::relation::rf_Relation;

use relax::random::generate_random;

#[no_mangle]
pub extern fn rf_relation_generate_random(s_ptr: *mut rf_Set, t_ptr: *mut rf_Set, p: c_double) -> *mut rf_Relation {
	let s = unsafe { s_ptr.as_ref() }.unwrap();
	let t = unsafe { t_ptr.as_ref() }.unwrap();
	let d = (s.clone(), t.clone());
	let r = generate_random(d, p);
	return Box::into_raw(Box::new(r));
}
