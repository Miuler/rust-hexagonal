use axum::{Router, Server};
use axum::routing::{get, post};

use auto_gpt_api::application::service::LlmService;

#[tokio::main]
async fn main() {
    let llm_service: LlmService = Default::default();

    let python_router = Router::new().route(
        "/execute",
        post(move |ai_settings: String| async move {
            llm_service.llm_execute(ai_settings.into()).await
        }),
    );

    let hello = Router::new().route("/", get(|| async { "Hello, World!" }));

    let api_routes = Router::new().nest("/", hello).nest("/api", python_router);

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(api_routes.into_make_service())
        .with_graceful_shutdown(async {
            println!("begin signal shutdown");
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
            println!("signal shutdown");
        })
        .await
        .unwrap();
}
