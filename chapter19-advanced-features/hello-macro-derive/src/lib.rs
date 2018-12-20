// Custom derive macro
extern crate proc_macro;  // RUST compiler API to manipulate source AST

use crate::proc_macro::TokenStream;
use quote::quote;  // tranform AST to RUST source code string
use syn;  // parse string of RUST source code to structure we can manipulate

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {  // called when #[derive(HelloMacro)]
        // Construct a representation of Rust code as AST that we can manipulate
        let ast = syn::parse(input).unwrap();

        // Build the trait implemention
        impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
                impl HelloMacro for #name {
                        fn hello_macro() {
                                println!("Hello macro.My name is {}.", stringify!(#name));
                        }
                }
        };
        gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
