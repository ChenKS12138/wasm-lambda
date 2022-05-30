#[macro_export]
macro_rules! dispatch_event {
    ($event:expr,[$($make_route:expr),*]) => {
        {
            let mut route_map = wasm_lambda_bridge::wasm_lambda_core::router::RouteMap::new();
            $(
                route_map.insert($make_route()).unwrap();
            )*

            let router:wasm_lambda_bridge::wasm_lambda_core::router::Router<_,_> = route_map.into();

            match $event {
                wasm_lambda_bridge::core::value::TriggerEvent::EventHttpRequest(request) => {
                    let split_idx:usize = request.path.find("?").unwrap_or(request.path.len());
                    let (handler, params) = router.search(&request.method,&request.path[..split_idx]).unwrap();
                    handler(wasm_lambda_bridge::core::value::TriggerEvent::EventHttpRequest(request),params)
                },
                wasm_lambda_bridge::core::value::TriggerEvent::EventInternalModuleCall(module_name,request) => {
                    let split_idx:usize = request.path.find("?").unwrap_or(request.path.len());
                    let (handler, params) = router.search(&request.method,&request.path[..split_idx]).unwrap();
                    handler(wasm_lambda_bridge::core::value::TriggerEvent::EventInternalModuleCall(module_name,request),params)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! make_headers {
    ($($key:expr => $value:expr),*) => {{
        let mut headers: std::collections::HashMap<String,String> = std::collections::HashMap::new();
        $(
            headers.insert($key.to_string(), $value.to_string());
        )*
        headers
    }}
}

#[macro_export]
macro_rules! make_response {
    ($body:expr) => {
        make_response!(200, $body)
    };
    ($status:expr,$body:expr) => {
        make_response!($status, wasm_lambda_bridge::make_headers!(), $body)
    };
    ($status:expr,$headers:expr,$body:expr) => {{
        Ok(wasm_lambda_bridge::core::value::Response {
            status: $status,
            headers: $headers,
            body: Some($body.try_into()?),
        })
    }};
}

#[macro_export]
macro_rules! make_json_response {
    ($body:expr) => {
        make_json_response!(200, $body)
    };
    ($status:expr,$body:expr) => {
        make_json_response!($status, wasm_lambda_bridge::make_headers!(), $body)
    };
    ($status:expr,$headers:expr,$body:expr) => {{
        wasm_lambda_bridge::make_response!(
            $status,
            {
                let mut headers = $headers;
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers
            },
            wasm_lambda_bridge::make_json_body!($body)
        )
    }};
}

#[macro_export]
macro_rules! make_json_body {
    ($value:expr) => {
        wasm_lambda_bridge::serde_json::to_string(&$value)?
    };
}
