use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hello_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident; // nom de la fonction
    let fn_name_str = fn_name.to_string(); //
    let output = quote! {
        #input

        fn hello_world() {
            println!("Hello, Wolrd! From {}", #fn_name_str);
        }
    };

    TokenStream::from(output)
}
