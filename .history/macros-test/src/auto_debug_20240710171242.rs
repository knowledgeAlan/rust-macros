use darling::{ast::Data,FromDeriveInput,FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;


#[derive(Debug,FromDeriveInput)]

struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data:Data<(),AutoDebugInfo>,
}


#[derive(Debug,FromField)];

