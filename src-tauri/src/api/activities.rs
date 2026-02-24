/// Activities endpoints for the Early API v4.
///
/// GET /activities returns all activities for the authenticated user,
/// including their IDs, names, and colors.
use serde_json::Value;

use super::auth::make_request;

/// Fetch all activities from the Early API.
/// Returns the full JSON response with the activities array.
pub async fn get_activities(token: &str) -> Result<Value, String> {
    make_request("GET", "/activities", token, None).await
}
