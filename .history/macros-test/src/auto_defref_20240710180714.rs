use core::panic;

use darling::{ast::Data,FromDeriveInput,FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;


#[derive(Debug,FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo{
    ident: syn::Ident,
    generics: syn::Generics,
    data: syn::Data<(),AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>
}


#[derive(Debug,FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}


pub (crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {

    let AutoDerefInfo {
        ident,
        generics,
        data:Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else{
        let (fd,ty) = if let Some(field) = field{
            match fields.iter().find(|f|f.ident.as_ref().unwrap() == &field) {
                Some(f) => (field,&f.ty),
                None => panic("field {:?} not found in the data structure", field),
            }
        }
    }
}