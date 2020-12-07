// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode character composition and decomposition utilities
//! as described in
//! [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).
//!
//! ```rust
//! extern crate unicode_normalization;
//!
//! use unicode_normalization::char::compose;
//! use unicode_normalization::UnicodeNormalization;
//!
//! fn main() {
//!     assert_eq!(compose('A','\u{30a}'), Some('Å'));
//!
//!     let s = "ÅΩ";
//!     let c = s.nfc().collect::<String>();
//!     assert_eq!(c, "ÅΩ");
//! }
//! ```
//!
//! # crates.io
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unicode-normalization = "0.1.8"
//! ```

#![deny(missing_docs, unsafe_code)]
#![doc(
    html_logo_url = "https://unicode-rs.github.io/unicode-rs_sm.png",
    html_favicon_url = "https://unicode-rs.github.io/unicode-rs_sm.png"
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate core;

extern crate tinyvec;

pub use crate::decompose::Decompositions;
pub use crate::quick_check::{
    is_nfc, is_nfc_quick, is_nfc_stream_safe, is_nfc_stream_safe_quick, is_nfd, is_nfd_quick,
    is_nfd_stream_safe, is_nfd_stream_safe_quick, is_nfkc, is_nfkc_quick, is_nfkd, is_nfkd_quick,
    IsNormalized,
};
pub use crate::recompose::Recompositions;
pub use crate::stream_safe::StreamSafe;
pub use crate::tables::UNICODE_VERSION;
use core::str::Chars;

mod no_std_prelude;

mod decompose;
mod lookups;
mod normalize;
mod perfect_hash;
mod quick_check;
mod recompose;
mod stream_safe;

#[rustfmt::skip]
mod tables;

#[doc(hidden)]
pub mod __test_api;
#[cfg(test)]
mod test;

/// Methods for composing and decomposing characters.
pub mod char {
    pub use crate::normalize::{
        compose, decompose_canonical, decompose_canonical_ext, decompose_compatible,
        decompose_compatible_ext,
    };

    pub use crate::lookups::{canonical_combining_class, is_combining_mark};
}

/// Methods for iterating over strings while applying Unicode normalizations
/// as described in
/// [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).
pub trait UnicodeNormalization<I: Iterator<Item = char>> {
    /// Returns an iterator over the string in Unicode Normalization Form D
    /// (canonical decomposition).
    fn nfd(self) -> Decompositions<I>;

    /// Returns an iterator over the string in Unicode Normalization Form KD
    /// (compatibility decomposition).
    fn nfkd(self) -> Decompositions<I>;

    /// An Iterator over the string in Unicode Normalization Form C
    /// (canonical decomposition followed by canonical composition).
    fn nfc(self) -> Recompositions<I>;

    /// An Iterator over the string in Unicode Normalization Form KC
    /// (compatibility decomposition followed by canonical composition).
    fn nfkc(self) -> Recompositions<I>;

    /// Similar to `nfd`, but with extensions which differ from the standard
    /// decomposition algorithm and which don't have a stability guarantee,
    /// but which still produce valid NFD and provide better results:
    ///  - Standardized Variation Seqeuences are used to avoid losing
    ///    information when normalizing "CJK Compatibility Ideographs"
    ///    codepoints. Note that many systemes today ignore variation
    ///    selectors, but the information is at least preserved in a
    ///    standardized form.
    ///
    /// Additional extensions may be added in future versions.
    ///
    /// If you need to match the standard `toNFD` algorithm exactly, or you
    /// need a stability guarantee, use `nfd` instead.
    fn nfd_ext(self) -> Decompositions<I>;

    /// Similar to `nfkd`, and the result is valid NFKD, but with the same
    /// extensions as `nfd`.
    ///
    /// If you need to match the standard `toNFKD` algorithm exactly, or you
    /// need a stability guarantee, use `nfd` instead.
    fn nfkd_ext(self) -> Decompositions<I>;

    /// Similar to `nfc`, and the result is valid NFC, but with the same
    /// extensions as `nfd`.
    ///
    /// If you need to match the standard `toNFC` algorithm exactly, or you
    /// need a stability guarantee, use `nfd` instead.
    fn nfc_ext(self) -> Recompositions<I>;

    /// Similar to `nfkc`, and the result is valid NFKC, but with the same
    /// extensions as `nfd`.
    ///
    /// If you need to match the standard `toNFKC` algorithm exactly, or you
    /// need a stability guarantee, use `nfd` instead.
    fn nfkc_ext(self) -> Recompositions<I>;

    /// An Iterator over the string with Conjoining Grapheme Joiner characters
    /// inserted according to the Stream-Safe Text Process (UAX15-D4)
    fn stream_safe(self) -> StreamSafe<I>;
}

impl<'a> UnicodeNormalization<Chars<'a>> for &'a str {
    #[inline]
    fn nfd(self) -> Decompositions<Chars<'a>> {
        decompose::new_canonical(self.chars())
    }

    #[inline]
    fn nfkd(self) -> Decompositions<Chars<'a>> {
        decompose::new_compatible(self.chars())
    }

    #[inline]
    fn nfc(self) -> Recompositions<Chars<'a>> {
        recompose::new_canonical(self.chars())
    }

    #[inline]
    fn nfkc(self) -> Recompositions<Chars<'a>> {
        recompose::new_compatible(self.chars())
    }

    #[inline]
    fn nfd_ext(self) -> Decompositions<Chars<'a>> {
        decompose::new_canonical_ext(self.chars())
    }

    #[inline]
    fn nfkd_ext(self) -> Decompositions<Chars<'a>> {
        decompose::new_compatible_ext(self.chars())
    }

    #[inline]
    fn nfc_ext(self) -> Recompositions<Chars<'a>> {
        recompose::new_canonical_ext(self.chars())
    }

    #[inline]
    fn nfkc_ext(self) -> Recompositions<Chars<'a>> {
        recompose::new_compatible_ext(self.chars())
    }

    #[inline]
    fn stream_safe(self) -> StreamSafe<Chars<'a>> {
        StreamSafe::new(self.chars())
    }
}

impl<I: Iterator<Item = char>> UnicodeNormalization<I> for I {
    #[inline]
    fn nfd(self) -> Decompositions<I> {
        decompose::new_canonical(self)
    }

    #[inline]
    fn nfkd(self) -> Decompositions<I> {
        decompose::new_compatible(self)
    }

    #[inline]
    fn nfc(self) -> Recompositions<I> {
        recompose::new_canonical(self)
    }

    #[inline]
    fn nfkc(self) -> Recompositions<I> {
        recompose::new_compatible(self)
    }

    #[inline]
    fn nfd_ext(self) -> Decompositions<I> {
        decompose::new_canonical_ext(self)
    }

    #[inline]
    fn nfkd_ext(self) -> Decompositions<I> {
        decompose::new_compatible_ext(self)
    }

    #[inline]
    fn nfc_ext(self) -> Recompositions<I> {
        recompose::new_canonical_ext(self)
    }

    #[inline]
    fn nfkc_ext(self) -> Recompositions<I> {
        recompose::new_compatible_ext(self)
    }

    #[inline]
    fn stream_safe(self) -> StreamSafe<I> {
        StreamSafe::new(self)
    }
}
