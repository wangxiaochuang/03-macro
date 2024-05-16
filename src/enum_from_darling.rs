use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantFields>,
}

#[derive(Debug, FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: darling::ast::Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).unwrap()
    else {
        panic!("not enum")
    };
    // println!("ident: {:#?}, generics: {:#?}", ident, generics);
    // println!("data: {:#?}", data);
    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            darling::ast::Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have one field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            #ident::#var(value)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #( #from_impls )*
    }
}
