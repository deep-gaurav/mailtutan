use std::sync::Arc;

use axum::{
    extract::{Multipart, State},
    response::Json,
    routing::post,
    Router,
};
use lettre::{address::Envelope, Address, Message};
use uuid::Uuid;

use crate::{models, AppState};

pub fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/messages", post(send_message))
        .with_state(state)
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Json<models::Message> {
    let mut message = models::Message::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let filename = field.file_name().map(|s| s.to_string());
        let content_type = field.content_type().map(|s| s.to_string());
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "from" => {
                message.sender = String::from_utf8(data.to_vec()).unwrap();
            }
            "to" => {
                message.recipients = String::from_utf8(data.to_vec())
                    .unwrap()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
            }
            "subject" => {
                message.subject = String::from_utf8(data.to_vec()).unwrap();
            }
            "text" => {
                message.plain = Some(String::from_utf8(data.to_vec()).unwrap());
            }
            "html" => {
                message.html = Some(String::from_utf8(data.to_vec()).unwrap());
            }
            "attachments" => {
                message.attachments.push(models::Attachment {
                    filename: filename.unwrap(),
                    file_type: content_type.unwrap(),
                    body: data.to_vec(),
                    cid: Uuid::new_v4().to_string(),
                });
            }
            _ => {}
        }
    }

    let mut builder = Message::builder()
        .from(message.sender.parse().unwrap())
        .subject(message.subject.clone());

    for recipient in message.recipients.clone() {
        builder = builder.to(recipient.parse().unwrap());
    }

    let mut multipart = lettre::message::MultiPart::mixed().build();

    if let Some(plain) = message.plain.clone() {
        multipart = multipart.singlepart(
            lettre::message::SinglePart::builder()
                .header(lettre::message::header::ContentType::TEXT_PLAIN)
                .body(plain),
        );
    }

    if let Some(html) = message.html.clone() { 
        multipart = multipart.singlepart(
            lettre::message::SinglePart::builder()
                .header(lettre::message::header::ContentType::TEXT_HTML)
                .body(html),
        );
    }


    for attachment in message.attachments.clone() {
        let content_type = attachment.file_type.parse().unwrap();
        multipart = multipart.singlepart(
            lettre::message::Attachment::new(attachment.filename)
                .body(attachment.body, content_type),
        );
    }

    let email = builder.multipart(multipart).unwrap();

    message.source = email.formatted();

    let msg = state.storage.write().unwrap().add(message);

    Json(msg)
}
