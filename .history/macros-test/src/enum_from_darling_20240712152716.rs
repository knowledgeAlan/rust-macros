use darling::{
    ast::{Data,Fields,Style},
    FromDeriveInput,FromField,FromVariant,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug,FromDeriveInput)]
struct EnumFromDarling {

    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants,()>,
}

#[derive(Debug,FromVariant)]
struct  EnumVariants {
    ident :syn::Ident,
    fields: Fields<EnumVariantFeilds>,
}


#[derive(Debug,FromField)]
struct  EnumVariantFeilds {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input :DeriveInput)->TokenStream {

    let EnumFromDarling {

        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    else{
        panic!("EnumFromDarling only works on enums");
    };


    let from_impls = data.iter().map(|variant|{

        let var: &syn::Ident = &variant.ident;
        let style: &Style = &variant.fields.style;
        
    });
}