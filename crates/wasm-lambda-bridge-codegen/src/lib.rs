use std::rc::Rc;

use proc_macro::TokenStream;
use quote::{self, spanned::Spanned};
use syn::{parse_macro_input, FnArg};

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

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

macro_rules! route_method {
    ($method:ident,$method_value:expr) => {
        #[proc_macro_attribute]
        pub fn $method(args: TokenStream, item: TokenStream) -> TokenStream {
            let input: syn::ItemFn = match syn::parse(item.clone()) {
                Ok(it) => it,
                Err(e) => return token_stream_with_error(item, e),
            };
            let input = Rc::new(input);
            let macro_args_ast = parse_macro_input!(args as syn::AttributeArgs);
            if macro_args_ast.is_empty() {
                return token_stream_with_error(item, syn::Error::new(input.__span(), "path required"));
            }

            let path = macro_args_ast.get(0).unwrap();

            let func_args = &input.clone().sig.inputs;
            let func_args: Vec<&FnArg> = func_args.iter().collect();

            if func_args.len() != 2 {
                return token_stream_with_error(
                    item,
                    syn::Error::new(
                        input.clone().sig.inputs.__span(),
                        "two arguments are required",
                    ),
                );
            };

            let event = func_args.get(0).unwrap();
            let params = func_args.get(1).unwrap();

            let func_name = input.sig.ident.clone();
            let block = &input.block;
            let output = &input.sig.output;

            quote::quote!(
                fn #func_name() -> wasm_lambda_bridge::wasm_lambda_core::router::Route<String,Box<fn(wasm_lambda_bridge::core::value::TriggerEvent,std::collections::HashMap<String,String>) -> wasm_lambda_bridge::core::Result<wasm_lambda_bridge::core::value::Response >>> {
                    wasm_lambda_bridge::wasm_lambda_core::router::Route::new(String::from($method_value),#path, Box::new(|#event , #params| #output {
                        #block
                    }))
                }
            ).into()
        }
    };
}

route_method!(get, "GET");
route_method!(post, "POST");
route_method!(put, "PUT");
route_method!(delete, "DELETE");
route_method!(patch, "PATCH");
route_method!(options, "OPTIONS");
route_method!(head, "HEAD");
route_method!(connect, "CONNECT");
route_method!(trace, "TRACE");
