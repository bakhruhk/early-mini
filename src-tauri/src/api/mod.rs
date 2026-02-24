/// API module — handles all communication with the Early (Timeular) v4 API.
///
/// Submodules:
/// - `auth`: Sign-in, token management, and authenticated request helper
/// - `tracking`: Current tracking state (GET /tracking)
/// - `activities`: Activity list (GET /activities)
pub mod auth;
pub mod tracking;
pub mod activities;
