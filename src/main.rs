use std::{env, error::Error, io::{Read, stdin}, sync::Arc, time::Duration};
use std::mem::ManuallyDrop;
use axum::{Json, Router, extract::State, http::Request, response::Response, routing::post};
use base64::{Engine, prelude::BASE64_STANDARD};
use region::Protection;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{Span, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{hash::stage_2_phc, state::ServerState};

mod hash;
mod state;

#[derive(Deserialize)]
struct Phash {
    stage_1_digest: String,
    salt: String,
}

async fn phash(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<Phash>,
) -> String {
    let stage_1_digest = BASE64_STANDARD.decode(payload.stage_1_digest).unwrap();
    let stage_1_digest: [u8; 32] = stage_1_digest.try_into().unwrap();

    let salt = BASE64_STANDARD.decode(payload.salt).unwrap();
    let salt: [u8; 32] = salt.try_into().unwrap(); // Now's your chance to be a [[BIG SALT]]

    stage_2_phc(stage_1_digest, salt, state)
}

fn router(argon_secret: Arc<[u8]>) -> Router {
    Router::new()
        .route("/phash", post(phash))
        .with_state(Arc::new(ServerState {
            argon_secret
        }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_request(|req: &Request<_>, span: &Span| {
                    span.in_scope(|| {
                        info!(
                            path = req.uri().path(),
                            "receiving request"
                        )
                    })
                })
                .on_response(|res: &Response, latency: Duration, _span: &Span| {
                    tracing::info!(
                        status = %res.status(),
                        latency_ms = %latency.as_millis(),
                        "response sent"
                    );
                }),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    format!(
                        "{}=debug,tower_http=debug,axum::rejection=trace",
                        env!("CARGO_CRATE_NAME")
                    )
                    .into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

    // Read secret from stdin (length should be of that defined in SECRET_LENGTH)
    let secret_len: usize = env::var("SECRET_LENGTH").unwrap().parse().unwrap();

    // DontDrop
    let mut secret = ManuallyDrop::new(Vec::<u8>::with_capacity(secret_len));
    let _guard = region::lock(secret.as_ptr(), secret.capacity())?;
    secret.resize(secret_len, 0);

    stdin().read_exact(&mut *secret)?;

    // Modifications after this point would be very problematic, so combine Arc (which is Deref only)
    // and memory protections

    unsafe {
        let page = region::query(secret.as_ptr()).unwrap();

        region::protect(page.as_ptr::<()>(), page.len(), Protection::READ)
            .unwrap();
    }

    let argon_secret = unsafe {
        // Try not to allocate more memory that must also be mlock'ed
        Arc::<[u8]>::from_raw(&raw const **secret)
    };

    // ---.-SE.-XS.-EX:SEXX
    let listener = TcpListener::bind("127.173.197.139:7399").await.unwrap();

    info!("Starting HTTP server on 127.173.197.139:7399...");

    axum::serve(
        listener,
        router(argon_secret)
    ).await
    .map_err(From::from)
}
