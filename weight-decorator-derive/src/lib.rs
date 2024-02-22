use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Expr;
use syn::FnArg;
use syn::Ident;
use syn::ItemFn;
use syn::Pat;

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

#[proc_macro_attribute]
pub fn weight_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse function
    let ast: ItemFn = syn::parse(item).expect("Failed to parse input as a function item!");
    let name = &ast.sig.ident;
    let fname = format!("weighted_{}", name);
    let varname = syn::Ident::new(&fname, name.span());
    let body = &ast.block;
    let inputs = &ast.sig.inputs;

    // Parse weight annotation
    let ast_attr: Ident =
        syn::parse(attr).expect("Failed to parse weight annotation as a function to evaluate!");

    let mut test: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> =
        syn::punctuated::Punctuated::new();
    for i in inputs {
        if let FnArg::Typed(a) = i {
            if let Pat::Ident(c) = *a.pat.clone() {
                test.push(c.ident);
            }
        }
    }

    let gen = quote! {
    fn #varname(#inputs) -> Weight{
        let _ = #body;
        #ast_attr(#test)
    }
    };
    gen.into()
}
