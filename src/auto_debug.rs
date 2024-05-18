use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), AutoDebugFieldsInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: darling::ast::Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeubg only works on structs");
    };

    let fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let skip = field.skip;
        if skip {
            quote! {}
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
        impl ::core::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()
            }
        }
    }

    // println!("{:#?}", fields);
}

// impl ::core::fmt::Debug for RespBulkString {
//     #[inline]
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         ::core::fmt::Formatter::debug_struct_field2_finish(
//             f,
//             "RespBulkString",
//             "inner",
//             &self.inner,
//             "nothing",
//             &&self.nothing,
//         )
//     }
// }
