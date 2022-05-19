// the rules is in following order:
//  - RUSTC ALLOW
//  - RUSTC WARNING
//  - CLIPPY
// rules not enabled:
//  - box_pointers
//  - missing_copy_implementations
//  - missing_debug_implementations
//  - missing_docs
//  - unreachable_pub
//  - unsafe_code
//  - unused_crate_dependencies,
//  - unused_qualifications
//  - unused_results
//  - variant_size_differences,
// TODO: remove `clippy::nursery` below because it raises `unnecessary structure
// name repetition` error in domain/model.rs
#![cfg_attr(
    feature = "cargo-clippy",
    deny(
        absolute_paths_not_starting_with_crate,
        deprecated_in_future,
        elided_lifetimes_in_paths,
        explicit_outlives_requirements,
        keyword_idents,
        macro_use_extern_crate,
        meta_variable_misuse,
        missing_abi,
        non_ascii_idents,
        noop_method_call,
        pointer_structural_match,
        semicolon_in_expressions_from_macros,
        single_use_lifetimes,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_op_in_unsafe_fn,
        unstable_features,
        unused_extern_crates,
        unused_import_braces,
        unused_lifetimes,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        bad_asm_style,
        bare_trait_objects,
        bindings_with_variant_name,
        cenum_impl_drop_cast,
        clashing_extern_declarations,
        coherence_leak_check,
        confusable_idents,
        const_evaluatable_unchecked,
        const_item_mutation,
        dead_code,
        deref_nullptr,
        drop_bounds,
        dyn_drop,
        ellipsis_inclusive_range_patterns,
        exported_private_dependencies,
        forbidden_lint_groups,
        function_item_references,
        illegal_floating_point_literal_pattern,
        improper_ctypes,
        improper_ctypes_definitions,
        incomplete_features,
        indirect_structural_match,
        inline_no_sanitize,
        invalid_doc_attributes,
        invalid_value,
        irrefutable_let_patterns,
        large_assignments,
        late_bound_lifetime_arguments,
        legacy_derive_helpers,
        mixed_script_confusables,
        mutable_borrow_reservation_conflict,
        nontrivial_structural_match,
        non_camel_case_types,
        non_fmt_panics,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        no_mangle_generic_items,
        overlapping_range_endpoints,
        path_statements,
        private_in_public,
        proc_macro_back_compat,
        proc_macro_derive_resolution_fallback,
        redundant_semicolons,
        renamed_and_removed_lints,
        stable_features,
        temporary_cstring_as_ptr,
        trivial_bounds,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        unaligned_references,
        uncommon_codepoints,
        unconditional_recursion,
        uninhabited_static,
        unknown_lints,
        unnameable_test_items,
        unreachable_code,
        unreachable_patterns,
        unstable_name_collisions,
        unsupported_calling_conventions,
        unused_allocation,
        unused_assignments,
        unused_attributes,
        unused_braces,
        unused_comparisons,
        unused_doc_comments,
        unused_features,
        unused_imports,
        unused_labels,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_unsafe,
        unused_variables,
        warnings,
        where_clauses_object_safety,
        clippy::all,
        clippy::cargo,
        clippy::pedantic
    ),
    allow(
        deprecated,
        clippy::future_not_send,
        clippy::module_name_repetitions,
        clippy::multiple_crate_versions
    )
)]

mod crd;
pub mod domain;
pub mod env;
pub mod error;
pub mod web;
