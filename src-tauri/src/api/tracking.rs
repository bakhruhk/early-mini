/// Tracking endpoints for the Early API v4.
///
/// GET /tracking returns the currently running timer, or an error
/// with status 404 if nothing is being tracked.
use chrono::Utc;
use serde_json::{json, Value};

use super::auth::make_request;

/// Return the current UTC time in the format the Early API expects.
fn now_iso() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
}

/// Fetch the current tracking state from the Early API.
/// Returns the full JSON response including activity info and start time.
pub async fn get_current_tracking(token: &str) -> Result<Value, String> {
    make_request("GET", "/tracking", token, None).await
}

/// Start tracking a specific activity.
/// POST /tracking/{activityId}/start — requires {"startedAt": "..."} body.
pub async fn start_tracking(token: &str, activity_id: &str) -> Result<Value, String> {
    let path = format!("/tracking/{activity_id}/start");
    let body = json!({ "startedAt": now_iso() });
    make_request("POST", &path, token, Some(body)).await
}

/// Stop the currently running timer.
/// POST /tracking/stop — requires {"stoppedAt": "..."} body.
pub async fn stop_tracking(token: &str) -> Result<Value, String> {
    let body = json!({ "stoppedAt": now_iso() });
    make_request("POST", "/tracking/stop", token, Some(body)).await
}

/// Update the note/description on the currently running tracker.
/// PATCH /tracking with body { "note": { "text": "..." } }
pub async fn update_note(token: &str, text: &str) -> Result<Value, String> {
    let body = serde_json::json!({
        "note": {
            "text": text,
            "tags": [],
            "mentions": []
        }
    });
    make_request("PATCH", "/tracking", token, Some(body)).await
}
