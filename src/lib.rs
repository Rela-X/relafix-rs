extern crate relax;

mod set;
mod relation;
mod text_io;
mod random;

pub use set::rf_set_element_new_string;
pub use set::rf_set_element_new_set;
pub use set::rf_set_new;
pub use set::rf_set_clone;
pub use set::rf_set_new_union;
pub use set::rf_set_new_intersection;
pub use set::rf_set_new_powerset;
pub use set::rf_set_equal;
pub use set::rf_set_is_subset;
pub use set::rf_set_get_cardinality;
pub use set::rf_set_free;
pub use relation::rf_relation_new;
pub use relation::rf_relation_clone;
pub use relation::rf_relation_new_union;
pub use relation::rf_relation_new_intersection;
pub use relation::rf_relation_new_concatenation;
pub use relation::rf_relation_new_complement;
pub use relation::rf_relation_new_converse;
pub use relation::rf_relation_equal;
pub use relation::rf_relation_is_homogeneous;
pub use relation::rf_relation_is_antisymmetric;
pub use relation::rf_relation_is_asymmetric;
pub use relation::rf_relation_is_difunctional;
pub use relation::rf_relation_is_equivalent;
pub use relation::rf_relation_is_irreflexive;
pub use relation::rf_relation_is_partial_order;
pub use relation::rf_relation_is_preorder;
pub use relation::rf_relation_is_reflexive;
pub use relation::rf_relation_is_symmetric;
pub use relation::rf_relation_is_transitive;
pub use relation::rf_relation_is_lattice;
pub use relation::rf_relation_is_sublattice;
pub use relation::rf_relation_is_lefttotal;
pub use relation::rf_relation_is_functional;
pub use relation::rf_relation_is_function;
pub use relation::rf_relation_is_surjective;
pub use relation::rf_relation_is_injective;
pub use relation::rf_relation_is_bijective;
pub use relation::rf_relation_free;
pub use text_io::rf_set_to_string;
pub use text_io::rf_relation_to_string;
pub use text_io::rf_relation_format_tex;
pub use text_io::rf_string_free;
pub use random::rf_relation_generate_random;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
