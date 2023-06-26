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
//! 
//! #[derive(IntType)]
//! #[repr(u8)]
//! enum Method {
//!     A = 1,
//!     B = 2,
//!     #[default]
//!     C = 3,
//! }
//! assert!(matches!(1.into(), Method::A));
//! assert!(matches!(0.into(), Method::C));
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, Meta, Type, Result, Error, spanned::Spanned};
use quote::quote;


#[proc_macro_derive(IntType, attributes(default, ))]
pub fn inttype(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    let ident = &item.ident;
    let ty = item.attrs.iter().find_map(|attr| {
        let Meta::List(ref meta_list) = attr.meta else {
            return None;
        };
        if !attr.path().is_ident("repr") {
            return None;
        }

        syn::parse2::<Type>(meta_list.tokens.clone()).ok()
    }).expect("no repr(inttype) provided");

    let mut default_var = None;
    let mut result = None;
    let var = item.variants.iter().map(|v| {
        if v.attrs.iter().find(|attr| {
            attr.path().is_ident("default")
        }).is_some() {
            if default_var.is_some() {

                result = Some(Error::new(v.span(), "Multiple default variables supplied! should be only one!")
                        .into_compile_error());
            }
            default_var = Some(&v.ident);
        }

        if !matches!(v.fields, syn::Fields::Unit) {
            result = Some(Error::new(v.span(), "every variant must be Unit kind, like `None`")
                        .into_compile_error());
        }
        &v.ident
    }).collect::<Vec<_>>();

    if let Some(ret) = result {
        return ret.into();
    }

    let mut token_stream = quote! {
        impl From<#ident> for #ty {
            fn from(value: #ident) -> Self {
                value as Self
            }
        }
    };

    let from = if let Some(default_var) = default_var {
        quote! {
            impl From<#ty> for #ident {
                fn from(value: #ty) -> Self {
                    #![allow(non_upper_case_globals)]
                    #(
                        const #var: #ty = #ident::#var as #ty;
                    )*
                    match value {
                        #( #var => Self::#var, )*
                        _ => Self::#default_var,
                    }
                }
            }
        }
    } else {
        quote! {
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
        }
    };

    token_stream.extend(from.into_iter());
    token_stream.into()
}