/// API module — handles all communication with the Early (Timeular) v4 API.
///
/// Submodules:
/// - `auth`: Sign-in, token management, and authenticated request helper
/// - `tracking`: Current tracking state, start/stop, note updates
/// - `activities`: Activity list (GET /activities)
/// - `entries`: Time entries for daily summaries
pub mod auth;
pub mod tracking;
pub mod activities;
pub mod entries;
