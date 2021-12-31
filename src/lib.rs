extern crate proc_macro;
use std::collections::HashSet;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{Ident, DataStruct, Generics, Type};

/// Derives the random value of a struct by giving a random value to all it's fields
#[proc_macro_derive(Rand)]
pub fn derive_distr (input: TokenStream) -> TokenStream { 
    let ast = syn::parse(input).unwrap();
    impl_distr(&ast)
}

fn impl_distr(ast: &syn::DeriveInput) -> TokenStream {
    let data = &ast.data;
    let gen = match data {
        syn::Data::Struct(data) => impl_distr_struct(ast, data),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    gen.into()
}

fn impl_distr_struct (ast: &syn::DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    if generics.params.len() == 0 {
        return impl_distr_wo_generics(name, data)
    }

    impl_distr_w_generics(name, generics, data)
}

fn impl_distr_w_generics (name: &Ident, generics: &Generics, data: &DataStruct) -> TokenStream {
    let ident = data.fields.iter()
        .map(|x| x.ident.as_ref().unwrap());

    let ty = data.fields.iter()
        .map(|x| &x.ty);

    let gen = quote! {
        impl #generics rand::distributions::Distribution<#name #generics> for rand::distributions::Standard where rand::distributions::Standard: rand::distributions::Distribution #generics {
            #[inline]
            fn sample<R: rand::Rng + ?Sized> (&self, rng: &mut R) -> #name #generics {
                #name::#generics {
                    #(
                        #ident: <Self as rand::distributions::Distribution<#ty>>::sample(self, rng),
                    )*
                }
            }
        }
    };

    gen.into()
}

fn impl_distr_wo_generics (name: &Ident, data: &DataStruct) -> TokenStream {
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

    gen.into()
}