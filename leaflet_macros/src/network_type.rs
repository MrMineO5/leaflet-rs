use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote_spanned, DeriveInput, Field, Fields, Type};

fn has_attr(field: &Field, name: &str) -> bool {
    field.attrs.iter().any(|a| a.path().is_ident(name))
}

fn generate_read(fields: &Fields) -> TokenStream {
    let read_iter = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        let action = match ty {
            Type::Path(p) => {
                if p.qself.is_some() {
                    return parse_quote_spanned! { field.span() => compile_error!("QSelf is not supported") }
                }

                let ident = p.path.get_ident().map(|i| i.to_string());

                match ident.as_deref() {
                    Some("bool") => quote!(buf.read_bool()),
                    Some("i8") => quote!(buf.read_byte()),
                    Some("u8") => quote!(buf.read_ubyte()),
                    Some("i16") => quote!(buf.read_short()),
                    Some("u16") => quote!(buf.read_ushort()),
                    Some("i32") => {
                        if has_attr(field, "varint") {
                            quote!(buf.read_var_int())
                        } else {
                            quote!(buf.read_int())
                        }
                    }
                    Some("i64") => quote!(buf.read_long()),
                    Some("String") => quote!(buf.read_string(32767)),
                    _ => quote!(buf.read_network_type::<#ty>()),
                }
            }
            _ => unimplemented!(),
        };

        quote!(
            let #name = #action?;
        )
    });

    let idents = fields.iter().map(|field| &field.ident);

    quote!(
        fn read(buf: &mut leaflet_network_buffer::McBuf) -> leaflet_network_buffer::BufferResult<Self> {
            #(#read_iter)*
            Ok(Self { #(#idents),* })
        }
    )
}

fn generate_write(fields: &Fields) -> TokenStream {
    let write_iter = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        let action = match ty {
            Type::Path(p) => {
                if p.qself.is_some() {
                    return parse_quote_spanned! { field.span() => compile_error!("QSelf is not supported") }
                }

                let ident = p.path.get_ident().map(|i| i.to_string());

                match ident.as_deref() {
                    Some("bool") => quote!(buf.write_bool(self.#name)),
                    Some("i8") => quote!(buf.write_byte(self.#name as i8)),
                    Some("u8") => quote!(buf.write_ubyte(self.#name)),
                    Some("i16") => quote!(buf.write_short(self.#name as i16)),
                    Some("u16") => quote!(buf.write_ushort(self.#name)),
                    Some("i32") => {
                        if has_attr(field, "varint") {
                            quote!(buf.write_var_int(self.#name))
                        } else {
                            quote!(buf.write_int(self.#name))
                        }
                    }
                    Some("i64") => quote!(buf.write_long(self.#name)),
                    Some("String") => quote!(buf.write_string(self.#name.as_str())),
                    _ => quote!(buf.write_network_type(&self.#name))
                }
            }
            _ => unimplemented!(),
        };

        quote!(
            #action;
        )
    });

    quote!(
        fn write(&self, buf: &mut leaflet_network_buffer::McBuf) {
            #(#write_iter)*
        }
    )
}

pub fn network_type_derive_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let data = &input.data;
    let fields = if let syn::Data::Struct(data) = data {
        &data.fields
    } else {
        return quote_spanned! { input.span() => compile_error!("Must be used on struct") };
    };

    let read = generate_read(fields);
    let write = generate_write(fields);

    quote!(
        impl NetworkType for #name {
            #read
            #write
        }
    )
}
