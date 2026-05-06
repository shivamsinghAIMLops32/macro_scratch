extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// 1. We define the traits here so they can be exported
pub trait CustomSerialize {
    fn serialize(&self) -> String;
}

pub trait CustomDeserialize {
    fn deserialize(s: &str) -> Self;
}

// 2. The custom derive for Serialize
#[proc_macro_derive(CustomSerialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident; // Gets the name of the struct (e.g., "Swap")

    // Generate the Rust code using the `quote!` macro
    let expanded = quote! {
        impl my_macros::CustomSerialize for #name {
            fn serialize(&self) -> String {
                // Easy method: just format the two integers as "a,b"
                format!("{},{}", self.a, self.b)
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

// 3. The custom derive for Deserialize
#[proc_macro_derive(CustomDeserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl my_macros::CustomDeserialize for #name {
            fn deserialize(s: &str) -> Self {
                // Easy method: split by comma and parse the strings back to i32
                let parts: Vec<&str> = s.split(',').collect();
                Self {
                    a: parts[0].parse().expect("Failed to parse 'a' as i32"),
                    b: parts[1].parse().expect("Failed to parse 'b' as i32"),
                }
            }
        }
    };

    TokenStream::from(expanded)
}