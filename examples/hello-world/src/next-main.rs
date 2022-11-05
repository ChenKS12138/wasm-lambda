use wasm_lambda_bridge::codegen;

pub fn main() {
    App::run();
}

struct App;

#[codegen::app]
impl App {
    #[get("/")]
    fn index() -> impl Responder {
        "Hello"
    }
}
