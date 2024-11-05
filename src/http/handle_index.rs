use anyhow::Result;
use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::errors::SupercellError;

pub async fn handle_index() -> Result<impl IntoResponse, SupercellError> {
    Ok(Json(json!({"ok": true})).into_response())
}
