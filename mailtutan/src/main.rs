use std::sync::Arc;

use clap::Parser;
use mailtutan_lib::*;

mod config;
use crate::config::StorageType;
use config::Config;
use mailtutan_lib::storage::Storage;
use std::sync::RwLock;

use tokio::sync::broadcast;
use tokio::{self, runtime::Builder, signal};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let state = {
        let storage: Box<RwLock<dyn Storage + 'static>> = Box::new(RwLock::new(storage::SmtpRelay::new(config.smtp_relay_server, config.smtp_relay_server_username, config.smtp_relay_server_password)));
        Arc::new(AppState {
            storage,
            channel: broadcast::channel(100).0,
            smtp_auth_username: config.smtp_auth_username.clone(),
            smtp_auth_password: config.smtp_auth_password.clone(),
            http_auth_username: config.http_username,
            http_auth_password: config.http_password,
        })
    };


    let smtp_server = smtp::Builder::new()
        .with_state(state.clone())
        .with_ssl(config.smtp_cert_path, config.smtp_key_path)
        .with_auth(config.smtp_auth_username.is_some() && config.smtp_auth_password.is_some())
        .bind((config.ip, config.smtp_port).into())
        .build();


    tokio::select! {

        _ = runtime.spawn(smtp_server.serve()) => {
        }
        _ = signal::ctrl_c() => {
        }
    }
    runtime.shutdown_background();
}
