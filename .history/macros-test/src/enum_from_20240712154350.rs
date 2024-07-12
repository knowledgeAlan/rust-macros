use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input:DeriveInput) -> TokenStream {

    let ident: syn::Ident = input.ident();
    let generics: syn::Generics = input.generics();

    let variants = match input.data  {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works for enums"),
    };


    let from_imps = variants.iter().map(|variant|{

        let var:&syn::Ident = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) => {

                if fields.unnamed.len() != 1 {
                    quote! {}
                }else {
                    
                }
            }
        }

    });
}
