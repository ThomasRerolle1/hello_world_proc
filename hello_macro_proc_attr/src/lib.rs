use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn, Lit, Meta};

#[proc_macro_attribute]
pub fn hello_world_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();

    //retrieve the attributes

    let attr = parse_macro_input!(attr as Meta);

    let mut name_value = String::from("Unknown");

    if let Meta::NameValue(name_value_meta) = attr {
        if name_value_meta.path.is_ident("name") {
            if let Expr::Lit(expr_lit) = name_value_meta.value {
                if let Lit::Str(lit_str) = expr_lit.lit {
                    name_value = lit_str.value();
                }
            }
        }
    }

    let output = quote! {
        #input

        fn hello_world_attr() {
            println!("Appel de la fonction: {}", #fn_name_str);
            println!("Valeur de l'attribut {}", #name_value);


        }
    };
    TokenStream::from(output)
}
