use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::debug;

use crate::{AppError, AppState, wasm_runtime::runtime::run_guest_component};

pub async fn handle(
    axum::extract::State(state): axum::extract::State<AppState>,
    wasm_bytes: Bytes,
) -> Result<Response, AppError> {
    debug!(
        "Received Wasm component with size: {} bytes",
        wasm_bytes.len()
    );

    match run_guest_component(state, wasm_bytes).await {
        Ok(output_string) => {
            // run_guest_component returns Ok(String) for both guest success
            // and guest-level errors that are handled within the Wasm module.
            // The distinction is logged by run_guest_component.
            // The HTTP response here will be 200 OK with the resulting string.
            Ok((StatusCode::OK, output_string).into_response())
        }
        Err(host_error) => {
            // This covers host-level errors like Wasmtime traps,
            // instantiation failures, or component loading issues.
            // AppError will convert this into an appropriate HTTP error response.
            Err(AppError(host_error))
        }
    }
}
