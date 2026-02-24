/// Tracking endpoints for the Early API v4.
///
/// GET /tracking returns the currently running timer, or an error
/// with status 404 if nothing is being tracked.
use serde_json::Value;

use super::auth::make_request;

/// Fetch the current tracking state from the Early API.
/// Returns the full JSON response including activity info and start time.
pub async fn get_current_tracking(token: &str) -> Result<Value, String> {
    make_request("GET", "/tracking", token, None).await
}

/// Start tracking a specific activity.
/// POST /tracking/{activityId}/start
pub async fn start_tracking(token: &str, activity_id: &str) -> Result<Value, String> {
    let path = format!("/tracking/{activity_id}/start");
    make_request("POST", &path, token, None).await
}

/// Stop the currently running timer.
/// POST /tracking/stop
pub async fn stop_tracking(token: &str) -> Result<Value, String> {
    make_request("POST", "/tracking/stop", token, None).await
}
