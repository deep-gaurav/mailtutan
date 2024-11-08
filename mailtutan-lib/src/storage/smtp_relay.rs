use lettre::{address::Envelope, transport::smtp::{authentication::Credentials, SmtpTransportBuilder}, Address, Message, SmtpTransport, Transport};

use super::Storage;

pub struct SmtpRelay {
    pub credentials: Credentials,
    pub server: String,
}

impl SmtpRelay {
    pub fn new(server:String, username:String, password: String) -> Self {
        Self{
            credentials: Credentials::new(username, password),
            server
        }
    }
}

impl Storage for SmtpRelay {
    fn list(&self) -> Vec<crate::models::Message> {
        return  vec![];
    }

    fn add(&mut self, message: crate::models::Message) -> crate::models::Message {
        let orig = message.clone();
        println!("Send to smtp {}", self.server);
        let mailer = SmtpTransport::relay(&self.server).unwrap().credentials(self.credentials.clone()).build();
        let from = message.sender.parse::<Address>().expect("from not valid");
        let envelope = Envelope::new(Some(from), message.recipients.into_iter().map(|e|e.parse().expect("to not valid")).collect()).unwrap();
        println!("Envelope {:?}", envelope);
        mailer.send_raw(
            &envelope,
            &message.source
        ).expect("Send failed");
        orig
    }

    fn get(&self, item: usize) -> Option<crate::models::Message> {
        return None
    }

    fn remove(&mut self, item: usize) {
        
    }

    fn size(&self) -> usize {
        0
    }

    fn delete_all(&mut self) {
       
    }
}