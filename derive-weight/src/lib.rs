use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprTuple, FnArg, Ident, ItemFn, Pat};

/// This is a procedural macro attribute function that implements the `derive_weight_expr` macro.
///
/// * `attr`: Represents the weight annotation that should be a valid Rust expression.
/// * `item`: Represents the function to which the macro is applied.
///
/// # Examples
///
/// ```
/// #[derive_weight_expr(Weight::from_parts(10, 10))]
/// fn do_something(i: u32, j: u32) -> u32 {
///     i * j
/// }
///
/// assert_eq!(weighted_do_something(1, 2), Weight::from_parts(10, 10));
/// ```
///
#[proc_macro_attribute]
pub fn derive_weight_expr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(item).expect("Failed to parse input as a function!");
    let name = &ast.sig.ident;
    let fname = format!("weighted_{}", name);
    let varname = syn::Ident::new(&fname, name.span());
    let body = &ast.block;
    let inputs = &ast.sig.inputs;

    let ast_attr: Expr =
        syn::parse(attr).expect("Failed to parse weight annotation as an expression to evaluate!");

    let gen = quote! {
    fn #varname(#inputs) -> Weight{
        let _ = #body;
        #ast_attr
    }
    };
    gen.into()
}

/// This is a procedural macro attribute function that implements the `derive_weight_fn` macro.
///
/// * `attr`: Represents the weight annotation that should be a valid Rust function.
/// * `item`: Represents the function to which the macro is applied.
///
/// # Examples
///
/// ```
/// #[allow(dead_code)]
/// fn weight_for_do_something(_i: u32, _j: u32) -> Weight {
///    if i == 0 {
///        Weight::from_parts(10, 10)
///    }
///    else {
///        Weight::from_parts(20, 20)
///    }
/// }
/// #[derive_weight_expr(weight_for_do_something)]
/// fn do_something(i: u32, j: u32) -> u32 {
///    if i == 0 {
///        i + j
///    }
///    else {
///        i/j
///    }
/// }
///
/// assert_eq!(weighted_do_something(0, 2), Weight::from_parts(10, 10));
/// assert_eq!(weighted_do_something(10, 2), Weight::from_parts(20, 20));
/// ```
///
#[proc_macro_attribute]
pub fn derive_weight_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(item).expect("Failed to parse input as a function!");
    let name = &ast.sig.ident;
    let fname = format!("weighted_{}", name);
    let varname = syn::Ident::new(&fname, name.span());
    let body = &ast.block;
    let inputs = &ast.sig.inputs;

    let ast_attr: Ident =
        syn::parse(attr).expect("Failed to parse weight annotation as a function!");
    let mut args: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    for i in inputs {
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

/// This is a procedural macro attribute function that implements the `derive_weight_result` macro.
///
/// * `attr`: Represents the weight annotation that should be a tuple (weight_ok, weight_err).
/// * `item`: Represents the function to which the macro is applied.
///
/// # Examples
///
/// ```
/// #[derive_weight_result((Weight::zero(), Weight::from_parts(10, 10)))]
/// fn do_something(i: u32, j: u32) -> u32 {
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
///
#[proc_macro_attribute]
pub fn derive_weight_result(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(item).expect("Failed to parse input as a function!");
    let name = &ast.sig.ident;
    let fname = format!("weighted_{}", name);
    let varname = syn::Ident::new(&fname, name.span());
    let body = &ast.block;
    let inputs = &ast.sig.inputs;

    let ast_attr: ExprTuple =
        syn::parse(attr).expect("Failed to parse weight annotation as a tuple expression");
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
