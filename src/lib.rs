// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(clippy::missing_errors_doc, warnings)]

mod args;
pub mod consts;
pub mod error;

pub use args::Args;
use error::Result;

#[allow(clippy::missing_const_for_fn, clippy::needless_pass_by_value)] //remove when `lib_main` impl'ed
pub fn lib_main(_args: Args) -> Result<()> {
    Ok(())
}
