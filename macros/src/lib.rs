#![feature(auto_traits)]
#![feature(negative_impls)]
mod abs;
mod enum_utils;
//mod packet;

extern crate proc_macro;
use proc_macro::TokenStream;
use crate::abs::ProcMacro;
use crate::enum_utils::{EnumFields, EnumValues};

#[proc_macro_derive(EnumFields, attributes(enum_field, ef))]
pub fn enum_fields_derive(input: TokenStream) -> TokenStream {
    EnumFields.or_compile_err(input.into()).into()
}

#[proc_macro_derive(EnumValues)]
pub fn enum_values_derive(input: TokenStream) -> TokenStream {
    EnumValues.or_compile_err(input.into()).into()
}