extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(LanguagePack, attributes(LanguageRules))]
pub fn derive_language_pack(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_language_pack(&ast)
}

fn impl_language_pack(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attribute = ast.attrs.first().unwrap();
    let language = match &attribute.meta {
        syn::Meta::List(expr) => &expr.tokens,
        _ => panic!(r#"#[LanguageRules = ".."] must be a valid language rule set."#),
    };

    if let syn::Data::Struct(_) = ast.data {
        let tokens = quote! {
            impl #name {
                pub fn new() -> Self {
                    let rules = #language.iter().cloned().collect();
                    let translit = Transliterator::new(rules);

                    Self { translit }
                }
            }
        };

        tokens.into()
    } else {
        panic!("#[derive(LanguagePack)] is only defined for structs, not for enums!");
    }
}

#[proc_macro_derive(FromLatin)]
pub fn derive_from_latin(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_from_latin(&ast)
}

#[proc_macro_derive(ToLatin)]
pub fn derive_to_latin(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_to_latin(&ast)
}

fn impl_from_latin(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let syn::Data::Struct(_) = ast.data {
        let tokens = quote! {
            impl FromLatin for #name {
                fn from_latin(&self, input: &str) -> String {
                    self.translit.translit(&input, true)
                }
            }
        };

        tokens.into()
    } else {
        panic!("#[derive(FromLatin)] is only defined for structs, not for enums!");
    }
}

fn impl_to_latin(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let syn::Data::Struct(_) = ast.data {
        let tokens = quote! {
            impl ToLatin for #name {
                fn to_latin(&self, input: &str) -> String {
                    self.translit.translit(&input, false)
                }
            }
        };

        tokens.into()
    } else {
        panic!("#[derive(ToLatin)] is only defined for structs, not for enums!");
    }
}
