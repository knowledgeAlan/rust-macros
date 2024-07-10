use darling::{ast::Data,FromDeriveInput,FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput};


#[derive(Debug,FromDeriveInput)]

struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data:Data<(),AutoDebugInfo>,
}


#[derive(Debug,FromField)]
#[darling(attributes(debug))]

struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}


pub (crate) fn process_auto_debug(input: DeriveInput) -> TokenStream{

    let AutoDebugFieldsInfo {
        ident,
        generics,
        data:Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else{
        panic!("AutoDebug only works on structs");
    }


    let fields = fields.iter().map(|field: &AutoDebugFieldsInfo|{

        let ident: &syn::Ident =  field.ident.as_ref().unwrap();
        let skip:bool = field.skip;
        
        if skip {
            quote!{};
        }else{

            quote! {
                .field(stringify!(#ident),&self.#ident)
            }
        }
    });


    quote !{
        impl ::core::fmt:debug for #ident #generics {
            #[inline]
            fn fmt(&self,f:&mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify($ident))
                #(#fields)*
                .finish()
            }
        }
    }

}
