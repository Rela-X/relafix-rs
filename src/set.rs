use rf_Set;
use Set;

// FIXME
#[no_mangle]
pub unsafe extern "C" fn rf_set_element_new_string(s: rf_Set) -> rf_Set {
	println!("set_element_new_string from rust");
	return rf_Set::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_element_new_set(s: rf_Set) -> rf_Set {
	println!("set_element_new_set from rust");
	return rf_Set::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_new(n: i32/*TODO, s: rf_Set*/) -> rf_Set {
	println!("set_new from rust");
	return rf_Set::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_clone(s: rf_Set) -> rf_Set {
	println!("set_clone from rust");
	return rf_Set::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_new_intersection(p: rf_Set, q: rf_Set) -> rf_Set {
	println!("set_intersection from rust");
	return rf_Set::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_new_powerset(s: rf_Set) -> rf_Set {
	println!("set_powerset from rust");
	return rf_Set::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_equal(p: rf_Set, q: rf_Set) -> bool {
	println!("set_equal from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_is_subset(p: rf_Set, q: rf_Set) -> bool {
	println!("set_is_subset from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_get_cardinality(s: rf_Set) -> i32 {
	println!("set_get_cardinality from rust");
	return 1;
}

#[no_mangle]
pub unsafe extern "C" fn rf_set_free(s: rf_Set) {
	println!("set_free from rust");
}
