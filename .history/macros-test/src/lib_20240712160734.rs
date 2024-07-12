mod auto_debug;
mod auto_deref;
mod enum_from_darling;
mod enum_from;

use auto_debug::process_auto_debug;
use auto_deref::process_auto_deref;
use enum_from::process_enum_from;
use enum_from_darling::process_enum_from_darling;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input:TokenStream) -> TokenStream {
    let input:DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    return process_enum_from(input).into();
}


#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input:TokenStream) -> TokenStream{
    let input:DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    process_enum_from_darling(input).into()
}

#[proc_macros_derive(AutoDeref,attributes(deref))]
pub fn derive_auto_deref(input:TokenStream) -> TokenStream{
    
}






