use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input:DeriveInput) -> TokenStream {

    let ident: syn::Ident = input.ident;
    let generics: syn::Generics = input.generics;

    let variants = match input.data  {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works for enums"),
    };


    let from_imps = variants.iter().map(|variant: &syn::Variant|{

        let var:&syn::Ident = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) => {

                if fields.unnamed.len() != 1 {
                    quote! {}
                }else {
                    let field:&syn::Field = fields.unnamed.first().expect("should have 1 field");
                    let ty:&syn::Type = &field.ty;

                    quote! {
                         impl #generics From<#ty> for #ident #generics{
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                         }   
                    }
                }
            }

            _ => quote! {},
        }

         

    });


    quote! {
        #(#from_impls)*
    }
}
