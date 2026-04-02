use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use syn::{Data, Fields};

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extraire les champs
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder only supports named fields"),
        },
        _ => panic!("Builder only supports structs"),
    };

    // Noms et types des champs
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let builder_name = quote::format_ident!("{}Builder", name);

    let expanded = quote! {
        pub struct #builder_name {
            #( #field_names: Option<#field_types> ),*
        }

        impl #builder_name {
            pub fn new() -> Self {
                Self {
                    #( #field_names: None ),*
                }
            }

            #(
                pub fn #field_names(mut self, val: #field_types) -> Self {
                    self.#field_names = Some(val);
                    self
                }
            )*

            pub fn build(self) -> #name {
                #name {
                    #( #field_names: self.#field_names.unwrap() ),*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }
    };

    TokenStream::from(expanded)
}
