/// Time entry endpoints for the Early API v4.
///
/// Used to fetch today's tracked time summary for the Expanded Mode.
use serde_json::Value;

use super::auth::make_request;

/// Fetch time entries for a given date range.
/// `GET /time-entries/{startISO}/{endISO}`
///
/// Used to calculate "Today: Xh Ym tracked" in the Expanded Mode.
pub async fn get_time_entries(token: &str, start: &str, end: &str) -> Result<Value, String> {
    let path = format!("/time-entries/{start}/{end}");
    make_request("GET", &path, token, None).await
}
