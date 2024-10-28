use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(FromEnv, attributes(env_prefix))]
pub fn from_env_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = if let syn::Data::Struct(data) = input.data {
        data.fields
            .iter()
            .map(|field: &syn::Field| {
                let field_name: &Option<syn::Ident> = &field.ident;
                let field_name_str =
                    field_name.as_ref().unwrap().to_string().to_uppercase();

                let env_var_name = get_env_var_name(field, field_name_str);

                quote! {
                #field_name:  std::env::var(#env_var_name)
                .expect(&format!("Environment variable `{}` not set", #env_var_name))
                .parse()
                .expect(&format!("Failed to parse `{}`", #env_var_name))
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

fn get_env_var_name(field: &syn::Field, field_name_str: String) -> String {
    field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("env_prefix"))
        .map(|attr| {
            let mut prefix = attr
                .parse_args::<LitStr>()
                .expect(&format!("Prefix for `{}` has a problem", field_name_str))
                .value();
            prefix.push_str(&field_name_str);
            prefix
        })
        .unwrap_or(field_name_str)
}
