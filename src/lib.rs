use proc_macro::TokenStream;
use quote::quote;

// proc mac crate
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", input);

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
    .into()
}

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(up: DirectionUp<T>) -> Self {
//         Direction::Up(up)
//     }
// }
