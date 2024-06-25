//! ## What is it?
//!
//! volaris-tools is a library used for managing the corecrypto logic behind volaris, and any applications that require easy integration with the volaris format.
//!
//! ## Security
//!
//! volaris-tools is built on top of volaris-crypto - which uses modern, secure and audited<sup>1</sup> AEADs for encryption and decryption.
//!
//! You may find the audits for both AES-256-GCM and XChaCha20-Poly1305 on [the NCC Group's website](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/).
//!
//! <sup>1</sup> Deoxys-II-256 does not have an official audit, so use it at your own risk
//!
//! ## Who uses Volaris-Tools?
//!
//! This library is implemented by [volaris](https://github.com/volar-is/volaris), a secure command-line file
//! encryption utility.
//!
//! This crate was made to separate the logic away from the end-user application.
//!
//! It also allows for more things to be built on top of the corecrypto functionality, such as a GUI application.
//!
//!
//! You can read more about volaris, volaris-crypto, volaris-tools and the technical details [in the project's main documentation](https://github.com/volar-is/volaris/)!
//!

// lints
#![forbid(unsafe_code)]
#![warn(
    rust_2018_idioms,
    non_ascii_idents,
    unstable_features,
    unused_imports,
    unused_qualifications,
    clippy::pedantic,
    clippy::all
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::needless_pass_by_value,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

pub mod decrypt;
pub mod encrypt;
pub mod erase;
pub mod erase_dir;
pub mod hash;
pub mod hasher;
pub mod header;
pub mod key;
pub mod overwrite;
pub mod pack;
pub mod storage;
pub mod unpack;

pub mod utils;
