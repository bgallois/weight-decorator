use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Ident;
use syn::ItemFn;
use syn::{Expr, Result};
use frame_support::pallet_prelude::Weight;

#[proc_macro_attribute]
pub fn weight(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse function
    let ast: ItemFn = syn::parse(item).expect("Failed to parse input as a function item!");
    let name = &ast.sig.ident;
    let fname = format!("weighted_{}", name);
    let varname = syn::Ident::new(&fname, name.span());
    let body = &ast.block;
    let inputs = &ast.sig.inputs;

    // Parse weight annotation
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
