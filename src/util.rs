use core::ops::RangeInclusive;

use syn::{spanned::Spanned, Error, ExprRange};
use crate::int_range_ext::*;

pub fn expr_to_range<T: Utils>(expr: &ExprRange) -> Result<RangeInclusive<T>, Error> 
    where <T as core::str::FromStr>::Err: core::fmt::Display,
{
    let start = match &expr.start {
        Some(expr) => match expr.as_ref() {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(int) => match int.base10_parse::<T>() {
                    Ok(n) => n,
                    Err(e) => return Err(Error::new(int.span(), format!("{e}"))),
                },
                _ => return Err(Error::new(expr.span(), "only integer literal allowed here")),
            },
            _ => return Err(Error::new(expr.span(), "only literal allowed here")),
        },
        None => T::min_(),
    };

    let end = match &expr.end {
        Some(expr) => match expr.as_ref() {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(int) => match int.base10_parse::<T>() {
                    Ok(n) => n,
                    Err(e) => return Err(Error::new(int.span(), format!("{e}"))),
                },
                _ => return Err(Error::new(expr.span(), "only integer literal allowed here")),
            },
            _ => return Err(Error::new(expr.span(), "only literal allowed here")),
        },
        None => T::max_(),
    };

    // println!("expr: {} {:?}", expr.into_token_stream(), expr.span());

    match &expr.limits {
        syn::RangeLimits::HalfOpen(_dot_dot) => {
            if expr.end.is_none() {
                if (start ..= end).is_empty() {
                    return Err(Error::new(expr.span(), "range is empty"));
                }
                Ok(start ..= end)
            } else {
                let r = (start..end).to_inclusive().map_err(|_| Error::new(expr.span(), "range is empty"))?;
                Ok(r)
            }
        },
        syn::RangeLimits::Closed(_dot_dot_eq) => {
            if (start ..= end).is_empty() {
                return Err(Error::new(expr.span(), "range is empty"));
            }
            Ok(start ..= end)
        },
    }
}


#[allow(unused)]
struct Dummy(i32);

pub(crate) struct RangeChecker {
    typ: String,
    ptr: core::ptr::NonNull<Dummy>,
}

impl core::fmt::Debug for RangeChecker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("RangeChecker");
        ds.field("typ", &self.typ);

        macro_rules! fuck {
            ($ident: ident) => {{
                type T = $ident;

                let substracter = self.ptr.as_ptr().cast::<RangeSubtracter<T>>();
                ds.field("ranges", unsafe{&*substracter});
            }};
        }

        match self.typ.as_str() {
            "u8" => fuck!(u8),
            "u16" => fuck!(u16),
            "u32" => fuck!(u32),
            "u64" => fuck!(u64),
            "u128" => fuck!(u128),
            "usize" => fuck!(usize),
            "i8" => fuck!(i8),
            "i16" => fuck!(i16),
            "i32" => fuck!(i32),
            "i64" => fuck!(i64),
            "i128" => fuck!(i128),
            "isize" => fuck!(isize),
            _ => {},
        }

        ds.finish()
    }
}

impl Drop for RangeChecker {
    fn drop(&mut self) {
        macro_rules! fuck {
            ($ident: ident) => {{
                type T = $ident;
                let _ = unsafe { Box::from_raw(self.ptr.as_ptr().cast::<RangeSubtracter<T>>()) };
            }};
        }
        match self.typ.as_str() {
            "u8" => fuck!(u8),
            "u16" => fuck!(u16),
            "u32" => fuck!(u32),
            "u64" => fuck!(u64),
            "u128" => fuck!(u128),
            "usize" => fuck!(usize),
            "i8" => fuck!(i8),
            "i16" => fuck!(i16),
            "i32" => fuck!(i32),
            "i64" => fuck!(i64),
            "i128" => fuck!(i128),
            "isize" => fuck!(isize),
            _ => {},
        }
    }
}

impl RangeChecker {
    pub fn new(typ: String) -> Result<Self, ()> {
        macro_rules! fuck {
            ($ident: ident) => {{
                type T = $ident;
                Box::into_raw(Box::new(RangeSubtracter::<T>::new(..).unwrap())).cast::<Dummy>()
            }};
        }
        let ptr = match typ.as_str() {
            "u8" => fuck!(u8),
            "u16" => fuck!(u16),
            "u32" => fuck!(u32),
            "u64" => fuck!(u64),
            "u128" => fuck!(u128),
            "usize" => fuck!(usize),
            "i8" => fuck!(i8),
            "i16" => fuck!(i16),
            "i32" => fuck!(i32),
            "i64" => fuck!(i64),
            "i128" => fuck!(i128),
            "isize" => fuck!(isize),
            _ => return Err(()),
        };
        let ptr = unsafe { core::ptr::NonNull::new_unchecked(ptr) };
        Ok(Self { typ, ptr })
    }

    pub fn is_empty(&self) -> bool {
        macro_rules! fuck {
            ($ident: ident) => {{
                type T = $ident;

                let substracter = self.ptr.as_ptr().cast::<RangeSubtracter<T>>();
                unsafe{(&*substracter).is_empty()}
            }};
        }

        match self.typ.as_str() {
            "u8" => fuck!(u8),
            "u16" => fuck!(u16),
            "u32" => fuck!(u32),
            "u64" => fuck!(u64),
            "u128" => fuck!(u128),
            "usize" => fuck!(usize),
            "i8" => fuck!(i8),
            "i16" => fuck!(i16),
            "i32" => fuck!(i32),
            "i64" => fuck!(i64),
            "i128" => fuck!(i128),
            "isize" => fuck!(isize),
            _ => true,
        }
    }

    pub fn substract(&mut self, expr: &ExprRange) -> Result<(), Error> {
        macro_rules! fuck {
            ($ident: ident) => {{
                type T = $ident;

                let r = expr_to_range::<T>(expr)?;
                let substracter = self.ptr.as_ptr().cast::<RangeSubtracter<T>>();
                unsafe{(&mut *substracter).substract(&r)}.map_err(|_| Error::new(expr.span(), "range duplicated"))?;
            }};
        }

        match self.typ.as_str() {
            "u8" => fuck!(u8),
            "u16" => fuck!(u16),
            "u32" => fuck!(u32),
            "u64" => fuck!(u64),
            "u128" => fuck!(u128),
            "usize" => fuck!(usize),
            "i8" => fuck!(i8),
            "i16" => fuck!(i16),
            "i32" => fuck!(i32),
            "i64" => fuck!(i64),
            "i128" => fuck!(i128),
            "isize" => fuck!(isize),
            _ => {},
        }
        Ok(())
    }

    pub fn expr_to_inclusive_expr(&self, expr: &ExprRange) -> Result<ExprRange, Error> {
        macro_rules! fuck {
            ($ident: ident) => {{
                type T = $ident;

                let r = expr_to_range::<T>(expr)?;
                Ok(syn::parse_str::<ExprRange>(format!("{r:?}").as_str())?)
            }};
        }

        match self.typ.as_str() {
            "u8" => fuck!(u8),
            "u16" => fuck!(u16),
            "u32" => fuck!(u32),
            "u64" => fuck!(u64),
            "u128" => fuck!(u128),
            "usize" => fuck!(usize),
            "i8" => fuck!(i8),
            "i16" => fuck!(i16),
            "i32" => fuck!(i32),
            "i64" => fuck!(i64),
            "i128" => fuck!(i128),
            "isize" => fuck!(isize),
            _ => Err(Error::new(expr.span(), "This is not possible!")),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_to_range() {
        let expr = syn::parse_str::<ExprRange>("1..=10").unwrap();
        let r = expr_to_range::<u32>(&expr).unwrap();
        assert_eq!(r, 1..=10);

        let expr = syn::parse_str::<ExprRange>("1..10").unwrap();
        let r = expr_to_range::<u32>(&expr).unwrap();
        assert_eq!(r, 1..=9);

        let expr = syn::parse_str::<ExprRange>("1..=1").unwrap();
        let r = expr_to_range::<u32>(&expr).unwrap();
        assert_eq!(r, 1..=1);

        let expr = syn::parse_str::<ExprRange>("1..=0").unwrap();
        let r = expr_to_range::<u32>(&expr);
        assert!(r.is_err());

        let expr = syn::parse_str::<ExprRange>("1..1").unwrap();
        let r = expr_to_range::<u32>(&expr);
        assert!(r.is_err());

        let expr = syn::parse_str::<ExprRange>("1..=1").unwrap();
        let r = expr_to_range::<u32>(&expr).unwrap();
        assert_eq!(r, 1..=1);

        let expr = syn::parse_str::<ExprRange>("1..=1").unwrap();
        let r = expr_to_range::<u8>(&expr).unwrap();
        assert_eq!(r, 1..=1);

        let expr = syn::parse_str::<ExprRange>("..10").unwrap();
        let r = expr_to_range::<u8>(&expr).unwrap();
        assert_eq!(r, 0..=9);

        let expr = syn::parse_str::<ExprRange>("..").unwrap();
        let r = expr_to_range::<u8>(&expr).unwrap();
        assert_eq!(r, 0..=255);
    }

    #[test]
    fn test_checker() {
        let mut checker = RangeChecker::new("u8".to_string()).unwrap();
        println!("{:?}", checker);

        let expr = syn::parse_str::<ExprRange>("1..=10").unwrap();
        checker.substract(&expr).unwrap();
        println!("{:?}", checker);

        let expr = syn::parse_str::<ExprRange>("5..=15").unwrap();
        assert!(checker.substract(&expr).is_err());

        let expr = syn::parse_str::<ExprRange>("20..30").unwrap();
        checker.substract(&expr).unwrap();
        println!("{:?}", checker);

        checker.substract(&syn::parse_str::<ExprRange>("0..=0").unwrap()).unwrap();
        println!("{:?}", checker);

        checker.substract(&syn::parse_str::<ExprRange>("30..").unwrap()).unwrap();
        println!("{:?}", checker);

        checker.substract(&syn::parse_str::<ExprRange>("11..20").unwrap()).unwrap();
        println!("{:?}", checker);

        assert!(checker.is_empty());
    }
    
}
