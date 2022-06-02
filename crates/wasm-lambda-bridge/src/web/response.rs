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
        wasm_lambda_bridge::core::value::Response {
            status: $status,
            headers: $headers,
            body: Some($body.try_into().unwrap()),
        }
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
        wasm_lambda_bridge::serde_json::to_string(&$value).unwrap()
    };
}
