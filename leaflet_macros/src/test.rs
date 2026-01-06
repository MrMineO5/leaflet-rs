use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::DeriveInput;
use syn::spanned::Spanned;

pub fn my_proc_impl(input: TokenStream) -> TokenStream {
    quote!(println!("Answer: {}", #input))
}

pub fn my_proc_derive_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let data = &input.data;
    let fields = if let syn::Data::Struct(data) = data {
        &data.fields
    } else {
        return quote_spanned! { input.span() => compile_error!("Must be used on struct") }
    };

    let iter = fields.iter().map(|field| &field.ident);

    quote!(
        impl Test for #name {
            fn test(&self) {
                println!("Hello from {}", stringify!(#name));
                #(
                    println!("{}: {:?}", stringify!(#iter), self.#iter);
                )*
            }
        }
    )
}
