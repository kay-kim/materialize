// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// BEGIN LINT CONFIG
// DO NOT EDIT. Automatically generated by bin/gen-lints.
// Have complaints about the noise? See the note in misc/python/materialize/cli/gen-lints.py first.
#![allow(clippy::style)]
#![allow(clippy::complexity)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::mutable_key_type)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::map_entry)]
#![allow(clippy::box_default)]
#![warn(clippy::bool_comparison)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::no_effect)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::zero_prefixed_literal)]
#![warn(clippy::borrowed_box)]
#![warn(clippy::deref_addrof)]
#![warn(clippy::double_must_use)]
#![warn(clippy::double_parens)]
#![warn(clippy::extra_unused_lifetimes)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_question_mark)]
#![warn(clippy::needless_return)]
#![warn(clippy::redundant_pattern)]
#![warn(clippy::redundant_slicing)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::single_component_path_imports)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::useless_asref)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::builtin_type_shadow)]
#![warn(clippy::duplicate_underscore_argument)]
#![warn(clippy::double_neg)]
#![warn(clippy::unnecessary_mut_passed)]
#![warn(clippy::wildcard_in_or_patterns)]
#![warn(clippy::collapsible_if)]
#![warn(clippy::collapsible_else_if)]
#![warn(clippy::crosspointer_transmute)]
#![warn(clippy::excessive_precision)]
#![warn(clippy::overflow_check_conditional)]
#![warn(clippy::as_conversions)]
#![warn(clippy::match_overlapping_arm)]
#![warn(clippy::zero_divided_by_zero)]
#![warn(clippy::must_use_unit)]
#![warn(clippy::suspicious_assignment_formatting)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_unary_op_formatting)]
#![warn(clippy::mut_mutex_lock)]
#![warn(clippy::print_literal)]
#![warn(clippy::same_item_push)]
#![warn(clippy::useless_format)]
#![warn(clippy::write_literal)]
#![warn(clippy::redundant_closure)]
#![warn(clippy::redundant_closure_call)]
#![warn(clippy::unnecessary_lazy_evaluations)]
#![warn(clippy::partialeq_ne_impl)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::transmutes_expressible_as_ptr_casts)]
#![warn(clippy::unused_async)]
#![warn(clippy::disallowed_methods)]
#![warn(clippy::disallowed_macros)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::from_over_into)]
// END LINT CONFIG

//! Internal utility libraries for Materialize.
//!
//! **ore** (_n_): the raw material from which more valuable materials are extracted.
//! Modules are included in this crate when they are broadly useful but too
//! small to warrant their own crate.

#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(nightly_doc_features, feature(doc_cfg))]

#[cfg_attr(nightly_doc_features, doc(cfg(feature = "test")))]
#[cfg(feature = "test")]
pub mod assert;
pub mod bits;
pub mod cast;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "cli")))]
#[cfg(feature = "cli")]
pub mod cli;
pub mod codegen;
pub mod collections;
pub mod display;
pub mod env;
pub mod error;
pub mod fmt;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
pub mod future;
pub mod graph;
pub mod hash;
pub mod hint;
pub mod id_gen;
pub mod iter;
pub mod lex;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "metrics")))]
#[cfg(feature = "metrics")]
pub mod metrics;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "network")))]
#[cfg(feature = "network")]
pub mod netio;
pub mod now;
pub mod option;
pub mod panic;
pub mod path;
pub mod permutations;
pub mod process;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "process")))]
pub mod result;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
pub mod retry;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "stack")))]
#[cfg(feature = "stack")]
pub mod stack;
pub mod stats;
pub mod str;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
pub mod task;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "test")))]
#[cfg(feature = "test")]
pub mod test;
pub mod thread;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "tracing_")))]
#[cfg(feature = "tracing_")]
pub mod tracing;
pub mod vec;

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "tracing_")]
    pub use tracing;
}
