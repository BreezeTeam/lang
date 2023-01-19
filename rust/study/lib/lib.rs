extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(PathFindOption)]
pub fn path_find_option_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_path_find_option(&ast)
}

fn impl_path_find_option(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PathFindOption for #name {}
    };
    gen.into()
}

