use proc_macro::TokenStream;
use quote::{self, spanned::Spanned};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    // if input.sig.ident != "main" {
    //     return token_stream_with_error(
    //         item,
    //         syn::Error::new(input.sig.ident.__span(), "should work with main"),
    //     );
    // }

    if let Some(event) = input.sig.inputs.first() {}

    // quote::quote!(
    //     #input
    // )
    // .into()
    // item
    quote::quote!(
        fn next_main() {}
    )
    .into()
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}
