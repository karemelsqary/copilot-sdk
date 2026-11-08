// SDK library entry point
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

/// Experimental runtime-owned Agent Host Protocol listeners.
pub mod ahp_host;
pub use ahp_host::{
    AhpAuthenticationRequest, AhpAuthenticationResult, AhpAuthenticator, AhpHost,
    AhpHostAuthentication, AhpHostExit, AhpHostExitCallback, AhpHostExitReason, AhpHostOptions,
    AhpSessionAuthentication, AhpSessionFactory, AhpSessionReleasedCallback, AhpSessionRequest,
    AhpSessionResumeFactory, AhpSessionResumeRequest,
};

#[cfg(any(test, has_bundled_cli, has_extracted_cli))]
mod cache_paths;
/// Canvas declarations, provider callbacks, and host-side canvas RPC types.
pub mod canvas;
mod canvas_dispatch;
#[cfg(feature = "local-runtime")]
