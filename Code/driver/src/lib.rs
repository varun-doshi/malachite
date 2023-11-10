//! Driver for the state machine of the Malachite consensus engine

#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_crate_dependencies, trivial_casts, trivial_numeric_casts)]
#![warn(
    // missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    variant_size_differences
)]
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::panic))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

mod driver;
mod env;
mod event;
mod message;
mod proposer;

pub use driver::Driver;
pub use env::Env;
pub use event::Event;
pub use message::Message;
pub use proposer::ProposerSelector;

// Re-export `#[async_trait]` macro for convenience.
pub use async_trait::async_trait;