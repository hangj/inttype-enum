#![doc=include_str!("../README.md")]

mod int_range_ext;
mod util;
use util::RangeChecker;

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Error, ExprRange, ItemEnum, Meta, Type};
use quote::{quote, ToTokens};


#[proc_macro_derive(IntType, attributes(default, ))]
pub fn inttype(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    let ident = &item.ident;
    let Some(ty) = item.attrs.iter().find_map(|attr| {
        let Meta::List(ref meta_list) = attr.meta else {
            return None;
        };
        if !attr.path().is_ident("repr") {
            return None;
        }

        syn::parse2::<Type>(meta_list.tokens.clone()).ok()
    }) else {
        // https://doc.rust-lang.org/reference/type-layout.html#primitive-representations
        return Error::new(item.span(), "no #[repr(inttype)] provided.\n`inttype` can be one of `u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, and isize`.\n See https://doc.rust-lang.org/reference/type-layout.html#primitive-representations")
        .into_compile_error().into();
    };

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
                type Error = #ty;
    
                fn try_from(value: #ty) -> Result<Self, Self::Error> {
                    #![allow(non_upper_case_globals)]
                    #(
                        const #var: #ty = #ident::#var as #ty;
                    )*
                    match value {
                        #( #var => Ok(Self::#var), )*
                        _ => Err(value)
                    }
                }
            }
        }
    };

    token_stream.extend(from.into_iter());
    token_stream.into()
}


#[proc_macro_derive(NewIntType, attributes(default, range))]
pub fn new_inttype(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    let ident = &item.ident;
    let Some(ty) = item.attrs.iter().find_map(|attr| {
        let Meta::List(ref meta_list) = attr.meta else {
            return None;
        };
        if !attr.path().is_ident("repr") {
            return None;
        }

        syn::parse2::<Type>(meta_list.tokens.clone()).ok()
    }) else {
        // https://doc.rust-lang.org/reference/type-layout.html#primitive-representations
        return Error::new(item.span(), "no #[repr(inttype)] provided.\n`inttype` can be one of `u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, and isize`.\nSee https://doc.rust-lang.org/reference/type-layout.html#primitive-representations")
        .into_compile_error().into();
    };

    let ty_str = ty.to_token_stream().to_string();

    // println!("ty: {}", ty_str);

    let mut checker = RangeChecker::new(ty_str.clone()).unwrap();
    let mut vars = Vec::with_capacity(item.variants.len());
    let mut ranges = Vec::with_capacity(item.variants.len());

    for v in item.variants.iter() {
        match &v.fields {
            syn::Fields::Named(_) => return Error::new(v.fields.span(), "variant can only be Unit/Unamed kind, Examples: A=0,B(u8),").into_compile_error().into(),
            //#[repr(u8)] #[derive(IntType)] enum { #[range(1..5)]a(u8), }
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return Error::new(fields.span(), &format!("Unnamed variant can only have 1 field, here it must be {}", ty.into_token_stream())).into_compile_error().into();
                }
                if v.discriminant.is_some() {
                    return Error::new(v.discriminant.as_ref().unwrap().1.span(), "Unnamed variant can't have discriminant").into_compile_error().into();
                }
                for unamed in fields.unnamed.iter() {
                    if unamed.ty.to_token_stream().to_string() != ty_str {
                        return Error::new(fields.span(), &format!("Unnamed variant's field must be the same as its repr: {}", ty.into_token_stream())).into_compile_error().into();
                    }
                }

                let mut range_cnt = 0;

                for attr in v.attrs.iter() {
                    if attr.path().is_ident("range") {
                        range_cnt += 1;
                        if range_cnt > 1 {
                            return Error::new(fields.span(), "Only one range expression must be provided for Unnamed variant").into_compile_error().into();
                        }
                        let range: ExprRange = match attr.parse_args() {
                            Ok(r) => r,
                            Err(e) => return e.into_compile_error().into(),
                        };

                        // println!("cur ident: {}", v.ident.to_string());

                        // println!("range: {}", range.to_token_stream());
                        if let Err(e) = checker.substract(&range) {
                            return e.into_compile_error().into();
                        }

                        ranges.push(range);
                    }
                }
                if range_cnt != 1 {
                    return Error::new(fields.span(), "one range expression must be provided for Unnamed variant").into_compile_error().into();
                }
            },
            //#[repr(u8)] #[derive(IntType)] enum { a=0, }
            syn::Fields::Unit => {
                // let s = v.ident.to_string();
                // println!("v: {}", v.to_token_stream());
                // println!("cur unit ident: {}", v.ident.to_string());
                match v.discriminant.as_ref() {
                    Some((_, n)) => {
                        let s = n.to_token_stream().to_string();
                        let range = syn::parse_str::<ExprRange>(format!("{}..={}", s, s).as_str()).unwrap();
                        if let Err(e) = checker.substract(&range) {
                            // println!("e.span(): {:?}", e.span());
                            return Error::new(n.span(), e.to_string()).into_compile_error().into();
                        }
                        ranges.push(range);
                    },
                    None => return Error::new(v.span(), "must specify discriminant, like A=0").into_compile_error().into(),
                }
            },
        }
        // println!("ident: {}", v.ident.to_string());
        vars.push(&v.ident);
    }

    println!("checker.is_empty(): {}", checker.is_empty());
    println!("ranges: {:?}", ranges.iter().map(|r| r.to_token_stream()).collect::<Vec<_>>());

    quote!{}.into()

    // let mut token_stream = quote! {
    //     impl From<#ident> for #ty {
    //         fn from(value: #ident) -> Self {
    //             value as Self
    //         }
    //     }
    // };

    // let from = if let Some(default_var) = default_var {
    //     quote! {
    //         impl From<#ty> for #ident {
    //             fn from(value: #ty) -> Self {
    //                 #![allow(non_upper_case_globals)]
    //                 #(
    //                     const #vars: #ty = #ident::#vars as #ty;
    //                 )*
    //                 match value {
    //                     #( #vars => Self::#vars, )*
    //                     _ => Self::#default_var,
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     quote! {
    //         impl TryFrom<#ty> for #ident {
    //             type Error = #ty;
    
    //             fn try_from(value: #ty) -> Result<Self, Self::Error> {
    //                 #![allow(non_upper_case_globals)]
    //                 #(
    //                     const #vars: #ty = #ident::#vars as #ty;
    //                 )*
    //                 match value {
    //                     #( #vars => Ok(Self::#vars), )*
    //                     _ => Err(value)
    //                 }
    //             }
    //         }
    //     }
    // };

    // token_stream.extend(from.into_iter());
    // token_stream.into()
}
