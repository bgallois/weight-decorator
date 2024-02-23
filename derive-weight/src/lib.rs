/*Copyright 2023 Benjamin Gallois

Use of this source code is governed by an MIT-style
license that can be found in the LICENSE file or at
https://opensource.org/licenses/MIT.*/

//! # Derive-weight
//!
//! `derive-weight` allows you to take any function with any signature, and a macro
//! will generate its equivalent with a Substrate Weight return signature.
//! It helps separate the Substrate pallet weight logic from the weight accounting
//! logic for functions running in handlers or in hooks.
//!

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprTuple, FnArg, Ident, ItemFn, Pat};

fn parse_fn(
    item: TokenStream,
) -> (
    syn::Ident,
    syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
    std::boxed::Box<syn::Block>,
) {
    let ast: ItemFn = syn::parse(item).expect("Failed to parse input as a function!");
    let name = &ast.sig.ident;
    let fname = format!("weighted_{}", name);
    let varname = syn::Ident::new(&fname, name.span());
    let body = &ast.block;
    let inputs = &ast.sig.inputs;
    (varname, inputs.clone(), body.clone())
}

fn derive_weight_expr(ast_attr: Expr, item: TokenStream) -> TokenStream {
    let (varname, inputs, body) = parse_fn(item);

    let gen = quote! {
    fn #varname(#inputs) -> Weight{
        let _ = #body;
        #ast_attr
    }
    };
    gen.into()
}

fn derive_weight_fn(ast_attr: Ident, item: TokenStream) -> TokenStream {
    let (varname, inputs, body) = parse_fn(item);

    let mut args: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    for i in &inputs {
        if let FnArg::Typed(a) = i {
            if let Pat::Ident(c) = *a.pat.clone() {
                args.push(c.ident);
            }
        }
    }

    let gen = quote! {
    fn #varname(#inputs) -> Weight{
        let _ = #body;
        #ast_attr(#args)
    }
    };
    gen.into()
}

fn derive_weight_result(ast_attr: ExprTuple, item: TokenStream) -> TokenStream {
    let (varname, inputs, body) = parse_fn(item);

    let success = &ast_attr.elems[0];
    let echec = &ast_attr.elems[1];

    let gen = quote! {
    fn #varname(#inputs) -> Weight{
        if let Ok(_i) = #body {
            #success
        }
        else {
            #echec
        }
    }
    };
    gen.into()
}

/// This is a procedural macro attribute function that implements the `derive_weight` macro.
///
/// * `attr`: Represents the weight annotation that should either a valid Rust expression,
/// a function returning Weight with the same signature that the decorated function or
/// a tuple with (weight_ok, weight_err) (see examples).
/// * `item`: Represents the function to which the macro is applied.
///
/// # Examples
///
/// ```
/// #[derive_weight(Weight::from_parts(10, 10))]
/// fn do_something(i: u32, j: u32) -> u32 {
///     i * j
/// }
///
/// assert_eq!(weighted_do_something(1, 2), Weight::from_parts(10, 10));
/// ```
/// ```
/// fn weight_for_do_something(i: u32, _j: u32) -> Weight {
///    if i == 0 {
///        Weight::from_parts(10, 10)
///    }
///    else {
///        Weight::from_parts(20, 20)
///    }
/// }
///
/// #[derive_weight(weight_for_do_something)]
/// fn do_something(i: u32, j: u32) -> u32 {
///    if i == 0 {
///        i + j
///    }
///    else {
///        i / j
///    }
/// }
///
/// assert_eq!(weighted_do_something(0, 2), Weight::from_parts(10, 10));
/// assert_eq!(weighted_do_something(10, 2), Weight::from_parts(20, 20));
/// ```
/// ```
/// #[derive_weight((Weight::zero(), Weight::from_parts(10, 10)))]
/// fn do_something(i: u32, j: u32) -> Ok((), ()) {
///    if i == 0 {
///        Ok(())
///    } else {
///        Err(())
///    }
/// }
///
/// assert_eq!(weighted_do_something(0), weight::from_parts(0, 0));
/// assert_eq!(weighted_do_something(1), weight::from_parts(10, 10));
/// ```
#[proc_macro_attribute]
pub fn derive_weight(attr: TokenStream, item: TokenStream) -> TokenStream {
    // The order matter.
    if let Ok(ast_attr) = syn::parse::<Ident>(attr.clone()) {
        derive_weight_fn(ast_attr, item)
    } else if let Ok(ast_attr) = syn::parse::<ExprTuple>(attr.clone()) {
        derive_weight_result(ast_attr, item)
    } else if let Ok(ast_attr) = syn::parse::<Expr>(attr.clone()) {
        derive_weight_expr(ast_attr, item)
    } else {
        panic!("Cannot parse the weight annotation.")
    }
}
