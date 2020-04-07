//! Procedural macro polyfill for Ok-wrapping functions.
//!
//! [Try blocks](https://doc.rust-lang.org/unstable-book/language-features/try-blocks.html)
//! are a nightly rust feature for introducing less-than-a-function boundary `?`
//! operator. One notable feature of try-blocks as currently implemented is
//! ok-wrapping:
//!
//! ```not-rust
//! let x: Option<i32> = try { 92 };
//! assert!(x.is_some());
//! ```
//!
//! That is, `try { expr }` desugars to, roughtly, `(|| Try::from_ok(expr))()`.
//!
//! **Crucially** you still have to use `?` to propagate erors, but you don't
//! have to wrap the final value in `Ok`. Moreover, if the final value is
//! already a `Result`, you need to add an extra `?`, to make the errors even
//! more explicit than without ok-wrapping.
//!
//! The try-block syntax naturarly generalizes to functions:
//!
//! ```not-rust
//! fn word_count(path: &Path) -> io::Result<usize> try {
//!     let mut res = 0;
//!     let file = fs::File::open(path)?;
//!     let mut reader = io::BufReader::new(file);
//!     for line in reader.lines() {
//!         let line = line?;
//!         res += line.split_whitespace().count();
//!     }
//!     res
//! }
//! ```
//!
//! Unfortunatelly, we can't have exact that even with proc-macros, but this
//! crate provides something close enough:
//!
//! ```
//! # use std::{path::Path, fs, io::{self, BufRead}};
//! use versuch::try_fn;
//!
//! ##[try_fn]
//! fn word_count(path: &Path) -> io::Result<usize> {
//!     let mut res = 0;
//!     let file = fs::File::open(path)?;
//!     let mut reader = io::BufReader::new(file);
//!     for line in reader.lines() {
//!         let line = line?;
//!         res += line.split_whitespace().count();
//!     }
//!     res
//! }
//! ```
//!
//! This crate is very much inspired by the [https://crates.io/crates/fehler](`fehler`) crate.
//!
//! Disclaimer: this crate is a proc macro and thus can measurably increase the
//! number of dependencies and compile times. It also breaks IDE support (sorry
//! for that, @matklad).
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

/// Wraps the body of the function into `Ok` or `Some`.
#[proc_macro_attribute]
pub fn try_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);

    let ok = ok(&sig).unwrap_or_else(|| panic!("only `Result` and `Option` are supported"));
    let ok = quote::format_ident!("{}", ok);

    TokenStream::from(quote::quote! {
        #(#attrs)* #vis #sig { #ok(#block) }
    })
}

fn ok(sig: &syn::Signature) -> Option<&'static str> {
    let path = match &sig.output {
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(path) => path,
            _ => return None,
        },
        _ => return None,
    };
    let last_segment = path.path.segments.last()?;
    let ident = &last_segment.ident;
    let res = if ident == "Result" {
        "Ok"
    } else if ident == "Option" {
        "Some"
    } else {
        return None;
    };
    Some(res)
}
