use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromEnv)]
pub fn from_env_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = if let syn::Data::Struct(data) = input.data {
        data.fields
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                let field_name_str = field_name.as_ref().unwrap().to_string().to_uppercase();

                quote! {
                    #field_name: std::env::var(#field_name_str)
                        .expect(&format!("Environment variable `{}` not set", #field_name_str))
                        .parse()
                        .expect(&format!("Failed to parse `{}`", #field_name_str))
                }
            })
            .collect::<Vec<_>>()
    } else {
        panic!("FromEnv can only be derived for structs");
    };

    let expanded = quote! {
        impl #struct_name {
            pub fn from_env() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
