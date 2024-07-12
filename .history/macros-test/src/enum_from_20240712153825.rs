use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input:DeriveInput) -> TokenStream {

    let ident = input.ident();
}
