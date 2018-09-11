use rf_Set;
use rf_Relation;

#[no_mangle]
pub unsafe extern "C" fn rf_relation_new(p: rf_Set, q: rf_Set /*TODO*/) -> rf_Relation {
	println!("relation_new from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_clone(r: rf_Relation) -> rf_Relation {
	println!("relation_clone from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_new_union(p: rf_Relation, q: rf_Relation) -> rf_Relation {
	println!("relation_new_union from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_new_intersection(p: rf_Relation, q: rf_Relation) -> rf_Relation {
	println!("relation_new_intersection from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_new_concatenation(p: rf_Relation, q: rf_Relation) -> rf_Relation {
	println!("relation_new_concatenation from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_new_complement(r: rf_Relation) -> rf_Relation {
	println!("relation_new_complement from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_new_converse(r: rf_Relation) -> rf_Relation {
	println!("relation_new_converse from rust");
	return rf_Relation::new();
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_homogeneous(r: rf_Relation) -> bool {
	println!("relation_is_homogeneous from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_antisymmetric(r: rf_Relation) -> bool {
	println!("relation_is_antisymmetric from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_asymmetric(r: rf_Relation) -> bool {
	println!("relation_is_asymmetric from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_difunctional(r: rf_Relation) -> bool {
	println!("relation_is_difunctional from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_equivalent(r: rf_Relation) -> bool {
	println!("relation_is_equivalent from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_irreflexive(r: rf_Relation) -> bool {
	println!("relation_is_irreflexive from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_partial_order(r: rf_Relation) -> bool {
	println!("relation_is_partial_order from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_preorder(r: rf_Relation) -> bool {
	println!("relation_is_preorder from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_reflexive(r: rf_Relation) -> bool {
	println!("relation_is_reflexive from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_symmetric(r: rf_Relation) -> bool {
	println!("relation_is_symmetric from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_transitive(r: rf_Relation) -> bool {
	println!("relation_is_transitive from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_lattice(r: rf_Relation) -> bool {
	println!("relation_is_lattice from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_sublattice(r: rf_Relation, s: rf_Relation) -> bool {
	println!("relation_is_sublattice from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_lefttotal(r: rf_Relation) -> bool {
	println!("relation_is_lefttotal from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_functional(r: rf_Relation) -> bool {
	println!("relation_is_functional from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_function(r: rf_Relation) -> bool {
	println!("relation_is_function from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_surjective(r: rf_Relation) -> bool {
	println!("relation_is_surjective from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_injective(r: rf_Relation) -> bool {
	println!("relation_is_injective from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_is_bijective(r: rf_Relation) -> bool {
	println!("relation_is_bijective from rust");
	return true;
}

#[no_mangle]
pub unsafe extern "C" fn rf_relation_free(r: rf_Relation) {
	println!("relation_free from rust");
}
