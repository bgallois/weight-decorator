use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprTuple, FnArg, Ident, ItemFn, Pat};

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
