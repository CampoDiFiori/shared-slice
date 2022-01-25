use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed, Type};

fn map_to_ref_wo_lifetime(t1: &syn::Type) -> syn::Type {
    match t1 {
        syn::Type::Reference(syn::TypeReference {
            elem, and_token, ..
        }) => match *elem.clone() {
            syn::Type::Slice(syn::TypeSlice {
                elem: t2,
                bracket_token,
            }) => syn::Type::Reference(syn::TypeReference {
                elem: Box::new(syn::Type::Slice(syn::TypeSlice {
                    elem: t2.clone(),
                    bracket_token: bracket_token.clone(),
                })),
                lifetime: None,
                mutability: None,
                and_token: *and_token,
            }),
            _ => unreachable!("Expected a slice of some T"),
        },
        _ => unreachable!("Expected a reference to a type"),
    }
}

fn map_to_type_inside_slice(t1: &syn::Type) -> syn::Type {
    match t1 {
        syn::Type::Reference(syn::TypeReference {
            elem,  ..
        }) => match *elem.clone() {
            syn::Type::Slice(syn::TypeSlice {
                elem: t2,
                ..
            }) => *t2.clone(),
            _ => unreachable!("Expected a slice of some T"),
        },
        _ => unreachable!("Expected a reference to a type"),
    }
}

#[proc_macro_derive(MultiSlice)]
pub fn multislice(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents_new_args = named.iter().map(|f| &f.ident);
                let idents_new_args1 = named.iter().map(|f| &f.ident);
                let idents_new_args2 = named.iter().map(|f| &f.ident);
                let idents_new_ret = named.iter().map(|f| &f.ident);
                let idents_getters = named.iter().map(|f| &f.ident);
                let idents_struct_def = named.iter().map(|f| &f.ident);
                let idents_capacity_def = named.iter().map(|f| &f.ident);
                let idents_transmutes = named.iter().map(|f| &f.ident);
                let idents_len_calc = named.iter().map(|f| &f.ident);

                let types_new_args = named.iter().map(|f| &f.ty).map(map_to_ref_wo_lifetime);
                let types_new_args1 = named.iter().map(|f| &f.ty).map(map_to_ref_wo_lifetime);
                let types_getters = named.iter().map(|f| &f.ty).map(map_to_ref_wo_lifetime);
                let types_inside_slices = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);
                let types_inside_slices2 = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);
                let types_inside_slices3 = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);

                let create_ident_len = |(i, f): (usize, &syn::Field)| {
                    let name = format!("len{}", i);
                    Ident::new(name.as_str(), Span::call_site())
                };

                let idents_len = named.iter().enumerate().map(create_ident_len);
                let idents_len_transmutes = named.iter().enumerate().map(create_ident_len);
                let idents_len_getters = named.iter().enumerate().map(create_ident_len);
                let idents_len_ret = named.iter().enumerate().map(create_ident_len);

                let first_ident = &named.first().expect("Expected at least one field in the struct").ident;
                let next_idents = named.iter().skip(1).map(|f| &f.ident);
                let idents_len_skip_1 = named.iter().take(next_idents.len()).enumerate().map(create_ident_len); 

                let multislice_ident =
                    Ident::new(format!("{}Multislice", ident).as_str(), Span::call_site());


                quote! {

                    pub struct #multislice_ident {
                        container: Vec<u8>,
                        _pin: std::marker::PhantomPinned,
                        #(
                        #idents_struct_def: *const u8,
                        #idents_len: usize,
                        )
                        *
                    }

                    impl #ident<'_> {
                        pub fn new(#(#idents_new_args1: #types_new_args1),*) -> #multislice_ident {
                            #multislice_ident::new(#(#idents_new_args2), *)
                        }
                    }

                    impl #multislice_ident {
                        fn new(#(#idents_new_args: #types_new_args), *) -> #multislice_ident {
                            let mut container = Vec::with_capacity(#(#idents_capacity_def.len() * std::mem::size_of::<#types_inside_slices2>() + )* 0);
                            let mut prev_slice_len = 0;

                            unsafe {
                                #(
                                let (_, p, _) = #idents_transmutes.align_to::<u8>();
                                container.extend_from_slice(p);
                                )
                                *
                            };

                            #(
                            let #idents_len_transmutes = #idents_len_calc.len();  
                            )
                            *

                            let mut ret = #multislice_ident {
                                container,
                                _pin: std::marker::PhantomPinned,
                                #(#idents_new_ret: std::ptr::null()), *,
                                #(#idents_len_ret), *
                            };

                            ret.#first_ident = ret.container.as_ptr();

                            unsafe {
                                #(
                                    ret.#next_idents = ret.container.as_ptr().offset(ret.#idents_len_skip_1 as isize);
                                )
                                *
                            }

                            ret
                        }

                        pub fn container(&self) -> &[u8] {
                            self.container.as_ref()
                        }

                        #(
                        pub fn #idents_getters(&self) -> #types_getters {
                            let ptr = self.#idents_getters as *const std::ffi::c_void;
                            let ptr = ptr as *const #types_inside_slices;
                            unsafe {
                                let slice = std::slice::from_raw_parts(ptr, self.#idents_len_getters);
                                // let (_, slice, _) = slice.align_to::<#types_inside_slices>(); 
                                // let reference: &str = std::str::from_utf8_unchecked(slice);
                                slice
                            }
                        }
                        )*
                    }
                }
            }
            _ => quote!(),
        },
        _ => quote!(),
    };

    output.into()
}
