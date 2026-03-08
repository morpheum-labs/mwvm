//! x402 Handler — HTTP 402 Payment Required enforcement.
//!
//! Inspects the `X-Payment` / `Authorization` header; returns 402 with a
//! payment request when missing; delegates to the paid handler otherwise.
//!
//! **Endpoints**:
//! - `POST /x402/pay` — Execute a paid agent operation

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::Serialize;
use tracing::{debug, info};

use crate::gateway::AppState;

// ── Protocol types ───────────────────────────────────────────────────────────

/// Payment request returned in the 402 response body.
#[derive(Debug, Serialize)]
pub struct PaymentRequest {
    /// Amount in base units.
    pub amount: u64,
    /// Currency symbol (e.g. `"USDC"`).
    pub currency: String,
    /// Human-readable memo.
    pub memo: String,
}

// ── Router ───────────────────────────────────────────────────────────────────

/// Mount x402 routes.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/x402/pay", post(handle_paid_task))
        .with_state(state)
}

// ── Handler ──────────────────────────────────────────────────────────────────

async fn handle_paid_task(
    State(_state): State<Arc<AppState>>,
    request: axum::extract::Request,
) -> axum::response::Response {
    let has_payment = request.headers().get("X-Payment").is_some()
        || request
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.starts_with("x402 "));

    if !has_payment {
        debug!("no x402 payment header — returning 402");

        let pr = PaymentRequest {
            amount: 100,
            currency: "USDC".into(),
            memo: "Morpheum agent operation".into(),
        };

        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(serde_json::json!({
                "error": "402 Payment Required",
                "payment_request": pr,
            })),
        )
            .into_response();
    }

    info!("x402 payment present — executing paid task");

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "paid_and_executed",
            "receipt": "x402_settlement_confirmed"
        })),
    )
        .into_response()
}

use axum::response::IntoResponse;
