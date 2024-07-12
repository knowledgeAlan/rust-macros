use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input:DeriveInput) -> TokenStream {

    let ident: syn::Ident = input.ident();
    let generics: syn::Generics = input.generics();

    let variants = match input.data  {
        sys::Data::Enum(data) => data.variants,
        
    };
}
