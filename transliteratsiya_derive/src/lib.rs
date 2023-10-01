extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token, Type};

#[derive(Debug)]
enum Item {
    Struct(ExprStruct),
}

#[derive(Debug)]
struct ExprStruct {
    ident: Ident,
    _brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse().map(Item::Struct)
        // let lookahead = input.lookahead1();
        // if lookahead.peek(Token![struct]) {
        //     input.parse().map(Item::Struct)
        // } else {
        //     Err(lookahead.error())
        // }
    }
}

impl Parse for ExprStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ExprStruct {
            ident: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn language_pack(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Item);
    let expanded = match input {
        Item::Struct(expr) => {
            let language = expr.ident;
            let mapping_field = expr
                .fields
                .iter()
                .find(|&f| f.ident.as_ref().unwrap().to_string() == "mapping")
                .unwrap();

            let pre_processor_mapping_field = expr
                .fields
                .iter()
                .find(|&f| f.ident.as_ref().unwrap().to_string() == "pre_processor_mapping");

            let mapping = match &mapping_field.ty {
                Type::Path(type_path) => {
                    let mapping = &type_path.path.segments.first().unwrap().ident;
                    quote! { #mapping.iter().cloned().collect() }
                }
                _ => panic!("Not a valid mapping for language {}", language),
            };

            let pre_processor_mapping = if let Some(field) = pre_processor_mapping_field {
                match &field.ty {
                    Type::Path(type_path) => {
                        let mapping = &type_path.path.segments.first().unwrap().ident;

                        quote! { Some(#mapping.iter().cloned().collect()) }
                    }
                    _ => panic!(
                        "Not a valid pre_processor_mapping for language {}",
                        language
                    ),
                }
            } else {
                quote! { None }
            };

            quote! {
                use crate::{transliterator::{FromLatin, ToLatin, Transliterator}, Language};

                #[derive(Debug)]
                pub struct #language {
                    translit: Transliterator
                }

                impl #language {
                    pub fn new() -> Self {
                        Self {
                            translit: Transliterator::new(#mapping, #pre_processor_mapping)
                        }
                    }
                }

                impl FromLatin for #language {
                    fn from_latin(&self, input: &str) -> String {
                        self.translit.translit(&input, true)
                    }
                }

                impl ToLatin for #language {
                    fn to_latin(&self, input: &str) -> String {
                        self.translit.translit(&input, false)
                    }
                }

                impl Language for #language {}
            }
        }
    };

    TokenStream::from(expanded)
}
