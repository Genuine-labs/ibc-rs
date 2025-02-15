#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types))]
#![deny(
    warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]
//! This library re-exports implementations of all the Inter-Blockchain
//! Communication (IBC) specifications available in [`ibc-rs`][ibc-rs]
//! repository. IBC is a distributed protocol that enables communication between
//! distinct sovereign blockchains.
//!
//! The layout of this crate mirrors the organization of the [IBC
//! Standard][ibc-standard]:
//!
//! + [Core](core) implements the transport, authentication, and ordering layers
//!   of the IBC protocol.
//!
//! + [Clients](clients) consists of implementations of client verification
//! algorithms (following the base client interface that is defined in `Core`)
//! for specific consensus algorithms. A chain uses these verification
//! algorithms to verify the state of remote chains.
//!
//! + [Applications](apps) consists of implementations of some IBC applications.
//! This is the part of the protocol that abstracts away the core protocol and
//! focuses solely on business logic.
//!
//! [ibc-standard]: https://github.com/cosmos/ibc
//! [ibc-rs]: https://github.com/cosmos/ibc-rs

#[cfg(any(test, feature = "std"))]
extern crate std;

/// Re-exports primitive types and traits from the `ibc-primitives` crate.
pub mod primitives {
    pub use ibc_primitives::*;
}

/// Re-exports implementations of all the IBC core (TAO) modules.
pub mod core {
    #[doc(inline)]
    pub use ibc_core::*;
}

/// Re-exports implementations of IBC light clients.
pub mod clients {
    #[doc(inline)]
    pub use ibc_clients::*;
}

/// Re-exports implementations of various IBC applications.
pub mod apps {
    #[doc(inline)]
    pub use ibc_apps::*;
}

/// Re-exports Cosmos-specific utility types, traits, and implementations.
pub mod cosmos_host {
    pub use ibc_core_host_cosmos::*;
}
