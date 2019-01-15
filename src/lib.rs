//! # relafix-rs
//!
//! `relafix-rs` is intended as a drop-in replacement of relafix.
//! It does so by providing a relafix-compatibility layer around librelax.
//! The library does not provide any functionality itself, but
//! only transforms arguments enough to pass them to librelax.
//!
//! A C-header containing all exported functions is generated using cbindgen.
mod set;
mod relation;
mod text_io;
mod random;

pub use crate::set::rf_set_element_new_string;
pub use crate::set::rf_set_element_new_set;
pub use crate::set::rf_set_new;
pub use crate::set::rf_set_clone;
pub use crate::set::rf_set_new_union;
pub use crate::set::rf_set_new_intersection;
pub use crate::set::rf_set_new_powerset;
pub use crate::set::rf_set_equal;
pub use crate::set::rf_set_is_subset;
pub use crate::set::rf_set_get_cardinality;
pub use crate::set::rf_set_free;
pub use crate::relation::rf_relation_new;
pub use crate::relation::rf_relation_clone;
pub use crate::relation::rf_relation_new_union;
pub use crate::relation::rf_relation_new_intersection;
pub use crate::relation::rf_relation_new_concatenation;
pub use crate::relation::rf_relation_new_complement;
pub use crate::relation::rf_relation_new_converse;
pub use crate::relation::rf_relation_equal;
pub use crate::relation::rf_relation_is_homogeneous;
pub use crate::relation::rf_relation_is_antisymmetric;
pub use crate::relation::rf_relation_is_asymmetric;
pub use crate::relation::rf_relation_is_difunctional;
pub use crate::relation::rf_relation_is_equivalent;
pub use crate::relation::rf_relation_is_irreflexive;
pub use crate::relation::rf_relation_is_partial_order;
pub use crate::relation::rf_relation_is_preorder;
pub use crate::relation::rf_relation_is_reflexive;
pub use crate::relation::rf_relation_is_symmetric;
pub use crate::relation::rf_relation_is_transitive;
pub use crate::relation::rf_relation_is_lattice;
pub use crate::relation::rf_relation_is_sublattice;
pub use crate::relation::rf_relation_is_lefttotal;
pub use crate::relation::rf_relation_is_functional;
pub use crate::relation::rf_relation_is_function;
pub use crate::relation::rf_relation_is_surjective;
pub use crate::relation::rf_relation_is_injective;
pub use crate::relation::rf_relation_is_bijective;
pub use crate::relation::rf_relation_free;
pub use crate::text_io::rf_set_to_string;
pub use crate::text_io::rf_relation_to_string;
pub use crate::text_io::rf_relation_format_tex;
pub use crate::text_io::rf_string_free;
pub use crate::random::rf_relation_generate_random;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
