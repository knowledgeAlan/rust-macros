use darling::{
    ast::{Data,Fields,Style},
    FromDeriveInput,FromField,FromVariant,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug,FromDeriveInput)]
struct EnumFromDarling {
    
}