use std::net::SocketAddr;

use axum::{
    extract::{MatchedPath, Path},
    http::{Request, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const FLAG_WEB_1: &'static str = env!("FLAG_WEB_1");
const MODEL_NUMBER: &'static str = "WL-ef2582640681ad8a1ac57583dcf98691ad";
const FIRMWARE_DL_URL: &'static str = "https://dl.nsec/slot-machine-66d23e935da38635924c1571ad165c5bd36ffe127eb579ec79e32e5226a1f136.tar.gz";
const PASSWORD: &'static str = "IHeardThatPassphrasesAreReallySecureAndAlsoBetterIfCustomizedSoThisIsTheServicePasswordToAccessWonderlightFirmwareFiles123$";

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // build our application with a route
    let app = Router::new()
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        // `GET /` goes to `root`
        .route("/", get(index))
        .route("/specs/{model_number}", get(specs));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(cli.bind_address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    Html(include_str!("client/index.html"))
}

async fn specs(Path(model_number): Path<String>) -> impl IntoResponse {
    if model_number != MODEL_NUMBER {
        return (
            StatusCode::NOT_FOUND,
            Html(include_str!("client/model_not_found.html").to_string()),
        );
    }

    let html = include_str!("client/specs.html")
        .replace("{{ flag }}", FLAG_WEB_1)
        .replace("{{ link }}", FIRMWARE_DL_URL)
        .replace("{{ password }}", PASSWORD);

    (StatusCode::FOUND, Html(html))
}
