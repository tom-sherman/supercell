use anyhow::Result;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::errors::SupercellError;

use super::context::WebContext;

pub async fn handle_well_known(
    State(web_context): State<WebContext>,
) -> Result<impl IntoResponse, SupercellError> {
    Ok(Json(json!({
         "@context": ["https://www.w3.org/ns/did/v1"],
         "id": format!("did:web:{}", web_context.external_base),
         "service": [
            {
                "id": "#bsky_fg",
                "type": "BskyFeedGenerator",
                "serviceEndpoint": format!("https://{}", web_context.external_base),
            }
         ]
    }))
    .into_response())
}
