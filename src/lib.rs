use axum::{response::Response as AxumResponse, routing::get, Router as AxumRouter};
use futures_executor::block_on;
use http::Request as HttpRequest;
use tower_service::Service; // unused import: `tower_service::Service`
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    // TODO: Convert Request -> HttpRequest
    let request: HttpRequest<String> = HttpRequest::builder()
        .uri("https://serverless.example/api/")
        .body("Some Body Data".into())
        .unwrap();

    let response: AxumResponse = block_on(app(request));

    // TODO: Convert AxumResponse -> Response
    Response::ok("Ok")
}

async fn app(request: HttpRequest<String>) -> AxumResponse {
    let router = AxumRouter::new()
        .route("/", get(|| async { "Hi!" }))
        .into_service(); // no method named `into_service` found for struct `axum::Router` in the current scope

    let response = router.call(request).await.unwrap();
    response
}
