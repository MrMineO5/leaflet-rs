use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{DeriveInput, LitInt};

pub fn packet_derive_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    if !matches!(input.data, syn::Data::Struct(_)) {
        return quote_spanned!(input.span() => compile_error!("Packet can only be derived for structs"););
    }

    let mut bound: Option<Ident> = None;
    for a in &input.attrs {
        if a.path().is_ident("serverbound") {
            if bound.is_some() {
                return quote_spanned!(a.span() => compile_error!("Duplicate bound attribute"););
            }
            bound = Some(Ident::new("Serverbound", a.span()));
        } else if a.path().is_ident("clientbound") {
            if bound.is_some() {
                return quote_spanned!(a.span() => compile_error!("Duplicate bound attribute"););
            }
            bound = Some(Ident::new("Clientbound", a.span()));
        }
    }
    let Some(bound) = bound else {
        return quote_spanned!(input.span() => compile_error!("Missing #[serverbound] or #[clientbound]"););
    };

    let state_attr = input.attrs.iter().find(|a| a.path().is_ident("state"));
    let Some(state_attr) = state_attr else {
        return quote_spanned!(input.span() => compile_error!("Missing #[state(...)]"););
    };
    let state: Ident = match state_attr.parse_args() {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let id_attr = input.attrs.iter().find(|a| a.path().is_ident("packet_id"));
    let Some(id_attr) = id_attr else {
        return quote_spanned!(input.span() => compile_error!("Missing #[packet_id(...)]"););
    };
    let id: LitInt = match id_attr.parse_args() {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    quote!(
        impl Packet for #name {
            const STATE: crate::ConnectionState = crate::ConnectionState::#state;
            const DIRECTION: crate::PacketDirection = crate::PacketDirection::#bound;
            const ID: i32 = #id;
        }
    )
}
