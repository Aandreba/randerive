extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{Ident, DataStruct, Generics, Data};

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

    let gen = quote! {
        use rand::distributions::{Standard, Distribution};

        impl #generics Distribution<#name #generics> for Standard where Standard: Distribution #generics {
            #[inline]
            fn sample<R: rand::Rng + ?Sized> (&self, rng: &mut R) -> #name #generics {
                #name::#generics {
                    #(
                        #ident: Self::sample(self, rng),
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
        use rand::distributions::{Standard, Distribution};

        impl Distribution<#name> for Standard {
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