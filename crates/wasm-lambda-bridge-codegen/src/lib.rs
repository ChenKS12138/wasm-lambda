use proc_macro::TokenStream;
use quote::{self, spanned::Spanned};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    let func_name = input.sig.ident.clone();
    if func_name != "main" {
        return token_stream_with_error(
            item,
            syn::Error::new(input.sig.ident.__span(), "should work with main"),
        );
    }

    let event = input.sig.inputs.first();
    if event.is_none() {
        return token_stream_with_error(
            item,
            syn::Error::new(
                input.sig.inputs.__span(),
                "at least one argument is required",
            ),
        );
    }
    let event = event.unwrap();
    let block = input.block;
    let output = input.sig.output;

    quote::quote!(
        fn #func_name() -> wasm_lambda_bridge::core::Result<()> {
            let #event = wasm_lambda_bridge::core::hostcall::event_recv().unwrap();
            let response = move || #output {
                #block
            }().unwrap();
            wasm_lambda_bridge::core::hostcall::event_reply(response).unwrap();
            Ok(())
        }
    )
    .into()
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}
