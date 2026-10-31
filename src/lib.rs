//#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    //missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(missing_docs, unused_variables, unused_imports, dead_code)]
#![doc = include_str!("../README.md")]

//***********************************************
// A/B Traits
//***********************************************
pub mod ab_traits;
#[doc(inline)]
pub use ab_traits::*;

//***********************************************
// A/B Test Other Types
//***********************************************
pub mod ab_types;
#[doc(inline)]
pub use ab_types::*;

//***********************************************
// Materials (certs, keys etc.)
// Exports const CA, KEY, CERT
//***********************************************
mod ab_materials;
pub(crate) use ab_materials::*;

//***********************************************
// Utilities relating to testing
//***********************************************
pub mod ab_util;
pub(crate) use ab_util::*;

//***********************************************
// Mock for Orbit Left/Right I/O
//***********************************************
pub mod ab_io_orbit;
#[doc(inline)]
pub use ab_io_orbit::*;

//***********************************************
// Mock for std::io::{Read, Write}
//***********************************************
pub mod ab_io_std;
#[doc(inline)]
pub use ab_io_std::*;

//***********************************************
// I/O Injector upon std Read call <> Orbit
//***********************************************
pub mod ab_injector;

//***********************************************
// Expected failure modes
//***********************************************
pub mod ab_failures;
#[doc(inline)]
pub use ab_failures::*;

//***********************************************
// TLS Under Test - Represented through Orbits
//***********************************************
pub mod ab_h11spec;
pub mod ab_rustls;
pub mod ab_ytls;

//***********************************************
// A/B TLS Tester Driver
//***********************************************
pub mod ab_tls_driver;
#[doc(inline)]
pub use ab_tls_driver::*;

//***********************************************
// A/B H11Spec Tester Driver
//***********************************************
pub mod ab_h11spec_driver;
#[doc(inline)]
pub use ab_h11spec_driver::*;
