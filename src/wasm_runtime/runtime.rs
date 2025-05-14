use anyhow::Result;
use axum::body::Bytes;
use tracing::{debug, error, info};
use wasmtime::{
    Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::AppState;

wasmtime::component::bindgen!({
    path: "wit",
    async: true,
    tracing: true,
});

struct HostComponent;

impl local::main::host::Host for HostComponent {
    async fn log(&mut self, msg: String) {
        info!("{}", msg);
    }
}

struct HostState {
    host: HostComponent,
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}
impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

pub async fn run_guest_component(state: AppState, wasm_bytes: Bytes) -> Result<String> {
    let mut store = Store::new(
        &state.engine,
        HostState {
            host: HostComponent {},
            wasi_ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            resource_table: ResourceTable::new(),
        },
    );
    store.set_fuel(500_000)?;

    let mut linker = Linker::new(&state.engine);
    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    local::main::host::add_to_linker(&mut linker, |state: &mut HostState| &mut state.host)?;

    debug!("Compiling Wasm component...");
    let component = Component::from_binary(&state.engine, &wasm_bytes)
        .map_err(|e| anyhow::format_err!("Failed to load component: {e}"))?;
    debug!("Compilation successful.");

    debug!("Instantiating component...");
    let runner_instance = Runner::instantiate_async(&mut store, &component, &linker)
        .await
        .map_err(|e| anyhow::format_err!("Failed to instantiate component: {e}"))?;
    debug!("Instantiation successful.");

    debug!("Calling guest run function...");
    let fuel_before = store.get_fuel().unwrap();
    let guest_execution_result = runner_instance.call_run(&mut store).await;
    let fuel_consumed = fuel_before - store.get_fuel().unwrap();

    let guest_result = match guest_execution_result {
        Ok(guest_app_level_result) => {
            match guest_app_level_result {
                Ok(success_payload) => {
                    debug!(
                        "Guest run function finished successfully. Result: {success_payload:?}.",
                    );
                    Ok(format!(
                        "Wasm component executed successfully. Output: {success_payload:?}",
                    ))
                }
                Err(guest_error) => {
                    error!("Guest application returned an error: {guest_error:?}.",);
                    // Return a client error (e.g., 400) or a server error (500)
                    // depending on the nature of guest_error.
                    Ok(format!("Guest application error: {guest_error:?}"))
                }
            }
        }
        Err(host_execution_error) => {
            Err(anyhow::anyhow!(
                "Host-level guest execution failed: {host_execution_error:?}.",
            ))
        }
    };

    debug!("Fuel consumed: {fuel_consumed}");

    guest_result
}
