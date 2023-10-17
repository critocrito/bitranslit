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

/// Simplify the construction of a language pack. The macro requires at least
/// one mapping and has additionally three optional mappings. Each mapping is an
/// array of tuples. The different mapping types are:
///
/// - `mapping` (required): Single character mappings that work both directions.
/// - `pre_processor_mapping` (optional): Mappings of single characters to multiple characters that work both directions.
/// - `reverse_specific_mapping` (optional): Single characters that are only applied in reverse transliterations.
/// - `reverse_specific_pre_processor_mapping` (optional): Mappings of multiple characters to singe characters that are only applied in reverse transliteration.
///
/// ```text
/// language_pack!(LanguageName {
///   mapping: ...,
///   pre_processor_mapping: ...,
///   reverse_specific_mapping: ...,
///   reverse_specific_pre_processor_mapping: ...,
/// });
///
/// ```
///
/// See the languages modules for real examples.
#[proc_macro]
pub fn language_pack(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Item);
    let expanded = match input {
        Item::Struct(expr) => {
            let language = expr.ident;
            let language_label = language.to_string().to_lowercase();
            let code_field = expr
                .fields
                .iter()
                .find(|&f| *f.ident.as_ref().unwrap() == "code")
                .unwrap();

            let mapping_field = expr
                .fields
                .iter()
                .find(|&f| *f.ident.as_ref().unwrap() == "mapping")
                .unwrap();

            let pre_processor_mapping_field = expr
                .fields
                .iter()
                .find(|&f| *f.ident.as_ref().unwrap() == "pre_processor_mapping");

            let reverse_specific_mapping_field = expr
                .fields
                .iter()
                .find(|&f| *f.ident.as_ref().unwrap() == "reverse_specific_mapping");

            let reverse_specific_pre_processor_mapping_field = expr
                .fields
                .iter()
                .find(|&f| *f.ident.as_ref().unwrap() == "reverse_specific_pre_processor_mapping");

            let code = match &code_field.ty {
                Type::Path(type_path) => {
                    let code = &type_path.path.segments.first().unwrap().ident;
                    quote! { #code }
                }
                _ => panic!("Not a valid language code for language {}", language),
            };

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

            let reverse_specific_mapping = if let Some(field) = reverse_specific_mapping_field {
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

            let reverse_specific_pre_processor_mapping =
                if let Some(field) = reverse_specific_pre_processor_mapping_field {
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
                use std::{convert::From, default:: Default};
                use crate::transliterator::{Transliterator, TransliteratorBuilder};

                #[derive(Clone, Debug)]
                pub struct #language {
                    language: String,
                    code: String,
                }

                impl Default for #language {
                    fn default() -> Self {
                        Self {
                            language: #language_label.to_string(),
                            code: #code.to_string()
                        }
                    }
                }

                impl #language {
                    pub fn new() -> Self {
                        Default::default()
                    }
                }

                impl From<#language> for Transliterator {
                    fn from(language: #language) -> Self {
                        TransliteratorBuilder::default()
                            .language(language.language)
                            .code(language.code)
                            .mapping(#mapping)
                            .pre_processor_mapping(#pre_processor_mapping)
                            .reverse_specific_mapping(#reverse_specific_mapping)
                            .reverse_specific_pre_processor_mapping(#reverse_specific_pre_processor_mapping)
                            .build()
                            .unwrap()
                    }
                }


            }
        }
    };

    TokenStream::from(expanded)
}
