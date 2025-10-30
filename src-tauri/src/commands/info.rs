// src/commands/info.rs
use crate::state::GlobalState;
use serde::Serialize;

#[derive(Serialize)]
pub struct PkgInfoResponse {
    pub version: String,
    pub initialized: bool,
    pub locale: String,
}

#[tauri::command]
pub async fn get_soft_info() -> Result<PkgInfoResponse, String> {
    let state = GlobalState::get();

    Ok(PkgInfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        initialized: state.app_states.is_initialized(),
        locale: sys_locale::get_locale().unwrap_or_else(|| "en".into()),
    })
}
