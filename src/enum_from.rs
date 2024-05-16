use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn process_enum_from(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let generics = input.generics;
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be used on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have one field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_) => quote! {},
        }
    });

    // quote return proc-macro2 TokenStream
    quote! {
        #(#from_impls)*
    }
}
