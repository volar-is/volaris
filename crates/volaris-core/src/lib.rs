//! ## What is it?
//!
//! Volaris-Core is a library used for managing cryptographic functions and headers that adhere to the Volaris format.
//!
//! ## Security
//!
//! Volaris-Core uses modern, secure and audited<sup>1</sup> AEADs for encryption and decryption.
//!
//! You may find the audits for both AES-256-GCM and XChaCha20-Poly1305 on [the NCC Group's website](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/).
//!
//! <sup>1</sup> Deoxys-II-256 does not have an official audit, so use it at your own risk
//!
//! ## Who uses Volaris-Core?
//!
//! This library is implemented by [Volaris](https://github.com/volar-is/Volaris), a secure multi-interface file
//! encryption utility.
//!
//! Volaris-Core makes it easy to integrate the Volaris format into your own projects (and if there's a feature that you'd like to see, please don't hesitate to [open a Github issue](https://github.com/volar-is/Volaris/issues)).
//!
//! You can read more about Volaris, Volaris-Core and the technical details [in the project's main documentation](https://github.com/volar-is/volaris/readme.md)!
//!
//! ## Thank you!
//!
//! Volaris-Core exclusively uses AEADs provided by the [RustCrypto Team](https://github.com/RustCrypto), so I'd like to give them a huge thank you for their hard work (this wouldn't have been possible without them!)
#![forbid(unsafe_code)]
#![warn(clippy::all)]

pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod cipher;
pub mod header;
pub mod key;
pub mod primitives;
pub mod protected;
pub mod stream;
pub use aead::Payload;
pub use zeroize::Zeroize;

#[cfg(feature = "visual")]
pub mod visual;
