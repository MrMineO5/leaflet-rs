use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use syn::{
    braced,
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, Path, Result, Token,
};

struct Input {
    groups: Vec<StateBlock>,
}

struct StateBlock {
    state: Ident,
    serverbound: Vec<Path>,
    clientbound: Vec<Path>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut groups = Vec::new();
        while !input.is_empty() {
            groups.push(input.parse()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { groups })
    }
}

impl Parse for StateBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let state: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut serverbound: Option<Vec<Path>> = None;
        let mut clientbound: Option<Vec<Path>> = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            let list_content;
            bracketed!(list_content in content);
            let items: Punctuated<Path, Token![,]> =
                list_content.parse_terminated(Path::parse_mod_style, Token!(,))?;
            let items_vec = items.into_iter().collect::<Vec<_>>();

            if key == "serverbound" {
                if serverbound.is_some() {
                    return Err(Error::new(key.span(), "duplicate `serverbound`"));
                }
                serverbound = Some(items_vec);
            } else if key == "clientbound" {
                if clientbound.is_some() {
                    return Err(Error::new(key.span(), "duplicate `clientbound`"));
                }
                clientbound = Some(items_vec);
            } else {
                return Err(Error::new(key.span(), "expected `serverbound` or `clientbound`"));
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            state,
            serverbound: serverbound.unwrap_or_default(),
            clientbound: clientbound.unwrap_or_default(),
        })
    }
}

/// Convert CamelCase => snake_case (simple, good enough for packet names)
fn camel_to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            for lc in ch.to_lowercase() {
                out.push(lc);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/// Derive handler method name from packet type path last segment:
/// - strip "Serverbound"/"Clientbound" prefix if present
/// - strip "Packet" suffix if present
/// - snake_case
/// - prepend "on_"
fn method_name_from_packet_path(p: &Path) -> Ident {
    let seg = p.segments.last().unwrap();
    let mut name = seg.ident.to_string();

    if let Some(rest) = name.strip_prefix("Serverbound") {
        name = rest.to_string();
    } else if let Some(rest) = name.strip_prefix("Clientbound") {
        name = rest.to_string();
    }

    if let Some(rest) = name.strip_suffix("Packet") {
        name = rest.to_string();
    }

    let snake = camel_to_snake(&name);
    format_ident!("on_{}", snake)
}

pub fn expand(input: TokenStream) -> TokenStream {
    let parsed: Input = match syn::parse2(input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let mut out = TokenStream::new();

    for group in parsed.groups {
        let state = group.state;

        // Generate for each bound separately
        for (bound_name, packets) in [
            ("Serverbound", group.serverbound),
            ("Clientbound", group.clientbound),
        ] {
            let trait_name = format_ident!("{}{}Handler", state, bound_name);
            let fn_name = format_ident!(
                "handle_{}_{}",
                camel_to_snake(&state.to_string()),
                bound_name.to_lowercase()
            );

            // Trait methods
            let methods = packets.iter().map(|pkt_ty| {
                let m = method_name_from_packet_path(pkt_ty);
                quote! { fn #m(&self, connection: &mut Self::ClientType, packet: #pkt_ty); }
            });

            // Dispatch arms
            let arms = packets.iter().map(|pkt_ty| {
                let m = method_name_from_packet_path(pkt_ty);
                quote! {
                    x if x == <#pkt_ty as Packet>::ID => {
                        let p = <#pkt_ty as leaflet_network_buffer::NetworkType>::read(buf)?;
                        handler.#m(connection, p)
                    }
                }
            });

            out.extend(quote! {
                pub trait #trait_name {
                    type ClientType;

                    #(#methods)*

                    fn on_unknown(&self, _connection: &mut Self::ClientType, _id: i32, _buf: &mut leaflet_network_buffer::McBuf) {}
                }

                pub fn #fn_name<H: #trait_name>(
                    connection: &mut H::ClientType,
                    buf: &mut leaflet_network_buffer::McBuf,
                    handler: &H,
                ) -> leaflet_network_buffer::BufferResult<()> {
                    let id = buf.read_var_int()?;
                    println!("Packet id: {id}");
                    match id {
                        #(#arms,)*
                        _ => handler.on_unknown(connection, id, buf),
                    }
                    Ok(())
                }
            });
        }
    }

    out
}
