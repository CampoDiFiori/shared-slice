use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

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
                let slice_idents = named.iter().map(|f| &f.ident);
                let slice_idents1 = named.iter().map(|f| &f.ident);
                let slice_idents2 = named.iter().map(|f| &f.ident);
                let slice_idents3 = named.iter().map(|f| &f.ident);
                let slice_idents4 = named.iter().map(|f| &f.ident);
                let slice_idents5 = named.iter().map(|f| &f.ident);

                let slice_types = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);
                let slice_types1 = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);
                let slice_types2 = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);
                let slice_types3 = named.iter().map(|f| &f.ty).map(map_to_type_inside_slice);

                let field_count = named.iter().len();

                let offsets_idx = named.iter().enumerate().map(|(i, _)| i);
                let offsets_idx2 = named.iter().enumerate().map(|(i, _)| i);

                let multislice_ident = Ident::new(format!("{}Multislice", ident).as_str(), Span::call_site());

                quote! {

                    pub struct #multislice_ident {
                        container: Vec<u8>,
                        offsets: [usize; #field_count],
                        _pin: std::marker::PhantomPinned,
                    }

                    impl #ident<'_> {
                        pub fn new(#(#slice_idents: &[#slice_types]),*) -> #multislice_ident {
                            #multislice_ident::new(#(#slice_idents1), *)
                        }
                    }

                    impl #multislice_ident {
                        fn new(#(#slice_idents2: &[#slice_types1]), *) -> #multislice_ident {
                            let mut container = Vec::with_capacity(#(#slice_idents3.len() * std::mem::size_of::<#slice_types2>() + )* 0);
                            
                            let mut offsets: [usize; #field_count] = [0; #field_count];

                            unsafe {
                                #(
                                offsets[#offsets_idx] = container.len();
                                let (_, p, _) = #slice_idents4.align_to::<u8>();
                                container.extend_from_slice(p);
                                )
                                *
                            };

                            #multislice_ident {
                                container,
                                offsets,
                                _pin: std::marker::PhantomPinned,
                            }
                        }

                        pub fn container(&self) -> &[u8] {
                            self.container.as_ref()
                        }

                        #(
                        pub fn #slice_idents5(&self) -> &[#slice_types3] {
                            let ptr = unsafe { self.container.as_ptr().offset(self.offsets[#offsets_idx2] as isize) } as *const std::ffi::c_void;
                            let ptr = ptr as *const #slice_types3;
                            let slice_len_in_bytes = if #offsets_idx2 + 1 == #field_count {
                                self.container.len() - self.offsets[#offsets_idx2]
                            } else {
                                self.offsets[#offsets_idx2 + 1] - self.offsets[#offsets_idx2]
                            };
                            let len_in_sizeof = slice_len_in_bytes / std::mem::size_of::<#slice_types3>();
                            unsafe {
                                let slice = std::slice::from_raw_parts(ptr, len_in_sizeof);
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
