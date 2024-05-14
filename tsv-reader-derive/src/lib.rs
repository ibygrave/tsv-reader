#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Fields};

fn constructors(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(ref fields) => {
            let field_vals = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!(#name: fields.parse_one()?)
            });
            quote!({ #(#field_vals),* })
        }
        Fields::Unnamed(ref fields) => {
            let field_vals = fields.unnamed.iter().map(|_| quote!(fields.parse_one()?));
            quote!((#(#field_vals),*))
        }
        Fields::Unit => quote!(),
    }
}

fn try_derive_read(input: DeriveInput) -> Result<proc_macro2::TokenStream, &'static str> {
    let type_name = input.ident;
    // Lifetime annoations on impl depend on whether the target type has a lifetime parameter.
    let (target_life, reader_life) = match input.generics.lifetimes().count() {
        0 => Ok((quote!(), quote!(<'_>))),
        1 => Ok((quote!(<'doc>), quote!(<'doc>))),
        _ => Err("Can't derive `Read` on type with more than one lifetime parameter"),
    }?;

    match input.data {
        syn::Data::Struct(ref data) => {
            let cons = constructors(&data.fields);
            Ok(quote!(Ok(Self #cons)))
        }
        syn::Data::Enum(ref data) => {
            let names = data.variants.iter().map(|var| &var.ident);
            let cons = data.variants.iter().map(|var| constructors(&var.fields));
            Ok(quote!(
            match fields.next_field()? {
                #(stringify!(#names) => Ok(Self::#names #cons),)*
                _ => Err(Error),
            }))
        }
        syn::Data::Union(_) => Err("Only structs can derive `Read`"),
    }
    .map(|body| {
        quote!(
        impl #target_life Read #reader_life for #type_name #target_life {
            fn parse_tsv(fields: &mut Walker #reader_life) -> Result<Self, Error> {
                #body
            }
        })
    })
}

/// Derive a parser for a type consisting of types implementing `Read`.
///
/// Unit types consume no input and construct an instance of the type.
///
/// Data structs (with named or anonymous fields) recursively call
/// `Read::parse_tsv` for each field in the order declared.
///
/// Enums consume one field which must be the name of one of the enum
/// variants. Any content of the variant is parsed in the same way as
/// a data struct.
#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let span = input.ident.span();

    match try_derive_read(input) {
        Ok(tks) => TokenStream::from(tks),
        Err(msg) => TokenStream::from(syn::Error::new(span, msg).to_compile_error()),
    }
}
