use std::rc::Rc;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{self, spanned::Spanned};
use syn::{parse_macro_input, FnArg, Local};

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

            let locals_template = {
                let foo_input: syn::ItemFn = syn::parse2(quote::quote! {
                    fn foo(){ let i:i32 = (&__event__,&__params__).into();}
                })
                .unwrap();
                match foo_input.block.stmts.get(0).unwrap() {
                    syn::Stmt::Local(local) => local.clone(),
                    _ => panic!("unexpected type"),
                }
            };

            let locals: Vec<Local> = func_args
                .iter()
                .map(|arg| match &arg {
                    FnArg::Typed(arg) => {
                        let mut locals = locals_template.clone();
                        locals.pat = syn::Pat::Type(arg.clone());
                        locals
                    }
                    _ => panic!("unexpected type"),
                })
                .collect();

            let func_name = input.sig.ident.clone();
            let block = &input.block;
            let output = &input.sig.output;

            quote::quote!(
                fn #func_name<'a>() -> wasm_lambda_bridge::web::router::RouteMap<'a,String,Box<wasm_lambda_bridge::web::Handler>, wasm_lambda_bridge::web::MiddlewareContext> {
                    let route = wasm_lambda_bridge::web::router::Route::new(String::from($method_value),#path, Box::new(|__event__ , __params__| #output {
                        #(#locals)*
                        drop(__event__);
                        drop(__params__);
                        {
                            #block
                        }
                    } as wasm_lambda_bridge::web::Handler));
                    let mut route_map:wasm_lambda_bridge::web::router::RouteMap<'a,String,Box<wasm_lambda_bridge::web::Handler>,wasm_lambda_bridge::web::MiddlewareContext> = wasm_lambda_bridge::web::router::RouteMap::new();
                    route_map.insert_route(route).unwrap();
                    route_map
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

#[derive(Default, FromMeta)]
#[darling(default)]
struct ResourceArgs {
    prefix: String,
    folder: String,
}

#[proc_macro]
pub fn static_resource(attrs: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as syn::AttributeArgs);

    let attrs = match ResourceArgs::from_list(&attrs) {
        Ok(v) => v,
        _ => panic!("attrs parse error"),
    };

    let entry: Vec<(String, String)> = walkdir::WalkDir::new(&attrs.folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|v| v.file_type().is_file())
        .map(|v| {
            (
                std::path::Path::new(&attrs.prefix)
                    .join(
                        v.path()
                            .strip_prefix(std::path::Path::new(&attrs.folder).file_name().unwrap())
                            .unwrap(),
                    )
                    .to_str()
                    .unwrap()
                    .to_string(),
                std::fs::canonicalize(v.path())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            )
        })
        .collect();

    let entry_token: Vec<proc_macro2::TokenStream> = entry
        .iter()
        .map(|(v1, v2)| {
            let resource_mime = new_mime_guess::from_path(v1).first_or_octet_stream().to_string();
            quote::quote!(
                route_map.insert_route(wasm_lambda_bridge::web::router::Route::new("GET".to_string(),#v1, Box::new(|__event__,__params__| -> wasm_lambda_bridge::core::Result<wasm_lambda_bridge::core::value::Response> {
                    wasm_lambda_bridge::make_response!(
                        200,
                        wasm_lambda_bridge::make_headers!(
                            "Content-Type" => #resource_mime
                        ),
                        include_bytes!(#v2).to_vec()
                    )
                } as wasm_lambda_bridge::web::Handler ))).unwrap();
            ).into()
        })
        .collect();

    quote::quote!(
        || -> wasm_lambda_bridge::web::router::RouteMap<'static,String,Box<fn(wasm_lambda_bridge::core::value::TriggerEvent,std::collections::HashMap<String,String>) -> wasm_lambda_bridge::core::Result<wasm_lambda_bridge::core::value::Response >>, wasm_lambda_bridge::web::MiddlewareContext > {
            let mut route_map = wasm_lambda_bridge::web::router::RouteMap::new();
            #(#entry_token)*
            route_map
        }
    )
    .into()
}
