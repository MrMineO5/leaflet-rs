use proc_macro::TokenStream;
use syn::parse_macro_input;

mod test;
mod network_type;
mod packet;
mod packet_handlers;

#[proc_macro]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    test::my_proc_impl(input).into()
}

#[proc_macro_derive(Test)]
pub fn my_proc_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    test::my_proc_derive_impl(input).into()
}

#[proc_macro_derive(NetworkType, attributes(varint))]
pub fn network_type_derive(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as syn::DeriveInput);
    network_type::network_type_derive_impl(input).into()
}

#[proc_macro_derive(Packet, attributes(packet_id, serverbound, clientbound, state))]
pub fn packet_derive(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as syn::DeriveInput);
    packet::packet_derive_impl(input).into()
}


#[proc_macro]
pub fn packet_handlers(input: TokenStream) -> TokenStream {
    packet_handlers::expand(input.into()).into()
}

