extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{Ident, DataStruct};

/// Derives the random value of a struct by giving a random value to all it's fields
#[proc_macro_derive(Rand)]
pub fn derive_distr (input: TokenStream) -> TokenStream { 
    let ast = syn::parse(input).unwrap();
    impl_distr(&ast)
}

fn impl_distr(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    let gen = match data {
        syn::Data::Struct(data) => impl_distr_struct(name, data),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    gen.into()
}

fn impl_distr_struct (name: &Ident, data: &DataStruct) -> TokenStream {
    let ident = data.fields.iter()
        .map(|x| x.ident.as_ref().unwrap());

    let gen = quote! {
        impl rand::distributions::Distribution<#name> for rand::distributions::Standard {
            #[inline]
            fn sample<R: rand::Rng + ?Sized> (&self, rng: &mut R) -> #name {
                #name {
                    #(
                        #ident: Self::sample(self, rng),
                    )*
                }
            }
        }
    };

    proc_macro::TokenStream::from(gen)
}