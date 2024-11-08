pub mod auth;
pub mod models;
pub mod smtp;
pub mod storage;

use std::sync::RwLock;
use storage::Storage;
use tokio::sync::broadcast::Sender;

pub struct AppState {
    pub storage: Box<RwLock<dyn Storage + 'static>>,
    pub channel: Sender<String>,
    pub smtp_auth_username: Option<String>,
    pub smtp_auth_password: Option<String>,
    pub http_auth_username: String,
    pub http_auth_password: String,
}
