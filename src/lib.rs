//! Converts an [`enum`] into an [`inttype`], and try to convert it back
//! 
//! Usage example:  
//! ```
//! #[derive(IntType)]
//! #[repr(u8)]
//! enum Cmd {
//!     Connect = 1,
//!     Bind = 2,
//!     Udp = 3,
//! }
//! 
//! let conn: u8 = Cmd::Connect.into();
//! assert!(matches!(Cmd::try_from(conn), Ok(Cmd::Connect)));
//! assert!(matches!(Cmd::try_from(0), Err(_)));
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, Meta, Type};
use quote::quote;


#[proc_macro_derive(IntType, attributes())]
pub fn inttype(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    let ident = &item.ident;
    let ty = item.attrs.iter().find_map(|attr| {
        let Meta::List(ref meta_list) = attr.meta else {
            return None;
        };
        let Some(ident) = meta_list.path.get_ident() else {
            return None;
        };
        if ident.to_string().as_str() != "repr" {
            return None;
        }

        syn::parse2::<Type>(meta_list.tokens.clone()).ok()
    }).expect("no repr(inttype) provided");

    let var = item.variants.iter().map(|v| {
        &v.ident
    }).collect::<Vec<_>>();

    quote! {
        impl From<#ident> for #ty {
            fn from(value: #ident) -> Self {
                value as Self
            }
        }

        impl TryFrom<#ty> for #ident {
            type Error = io::Error;

            fn try_from(value: #ty) -> Result<Self, Self::Error> {
                #![allow(non_upper_case_globals)]
                #(
                    const #var: #ty = #ident::#var as #ty;
                )*
                match value {
                    #( #var => Ok(Self::#var), )*
                    _ => Err(Self::Error::new(
                        io::ErrorKind::Unsupported, 
                        format!("unsupported value: {value}"),
                    ))
                }
            }
        }
    }.into()
}