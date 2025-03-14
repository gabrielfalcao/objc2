//! # Bindings to the `IOKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/iokit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-io-kit/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "libc")]
mod consumes_argument;
mod generated;

#[cfg(feature = "libc")]
#[allow(unused_imports, unreachable_pub)]
pub use self::consumes_argument::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// IOKit/IOReturn.h
/// [Apple's documentation](https://developer.apple.com/documentation/iokit/ioreturn?language=objc)
pub type IOReturn = core::ffi::c_int; // kern_return_t

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;

// device/device_types.h
#[allow(dead_code, non_camel_case_types)]
pub(crate) type io_name_t = *mut [core::ffi::c_char; 128];
#[allow(dead_code, non_camel_case_types)]
pub(crate) type io_string_t = *mut [core::ffi::c_char; 512];
#[allow(dead_code, non_camel_case_types)]
pub(crate) type io_struct_inband_t = *mut [core::ffi::c_char; 4096];
