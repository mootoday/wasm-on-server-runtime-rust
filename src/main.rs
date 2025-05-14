use anyhow::Result;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;
use wasmtime::{Config, Engine, WasmBacktraceDetails};

mod server;
mod wasm_runtime;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Error occurred: {:#}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Clone)]
pub struct AppState {
    engine: Engine,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new(
        Config::new()
            .async_support(true)
            .consume_fuel(true)
            .debug_info(true)
            .wasm_component_model(true)
            .wasm_backtrace_details(WasmBacktraceDetails::Enable),
    )?;

    let app_state = AppState { engine };
    server::start::start(app_state).await?;
    Ok(())
}
