use darling::{ast::Data,FromDeriveInput,FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;


#[derive(Debug,FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDebugInfo{
    ident: syn::Ident,
    generics: syn::Generics,
    data: syn::Data<(),AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>
}