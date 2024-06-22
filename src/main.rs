use database::User;
use itertools::Itertools;
use mailjet_rs::common::{Payload, Recipient};
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use serde_json::{to_string, Map, Value};
use std::env;
use uuid::Uuid;
use tokio_postgres::Error;

mod database;

//this program will be called every hour to check unread messages
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(
        SendAPIVersion::V3,
        env::var("MAILJET_PUBLIC_KEY").unwrap().as_str(),
        env::var("MAILJET_PRIVATE_KEY").unwrap().as_str(),
    );
    let emails = build_emails().await?;

    println!("Checking for emails to send...");

    if emails.messages.is_empty() {
        println!("No emails to send");
    } else {
        database::insert_unsubscribe_tokens(&emails.messages).await?;
        println!("Mailjet response {:?}", client.send(emails).await.expect("Could not send emails!"));
    }

    Ok(())
}

async fn build_emails() -> Result<Batch, Error> {
    let users = database::select_users_with_unread_messages().await?;
    let mut emails = Vec::new();

    for (id, chunk) in &users.into_iter().chunk_by(|elt| elt.id) {
        let rows: Vec<User> = chunk.collect();
        let mut user_names: Vec<String> = Vec::new();

        for r in &rows {
            user_names.push(format!(" <i>{}</i>", r.name));
        }

        let count = rows[0].unread_messages;
        let email_address = &rows[0].email;
        let mut message = Message::new(
            "contact@mero.chat",
            "MeroChat",
            Some("You have unread messages on MeroChat!".to_string()),
            None
        );
        let email_id = Uuid::new_v4().to_string();
        let mut vars = Map::new();

        vars.insert(String::from("unread_count"), Value::from(count));
        vars.insert(String::from("user_names"), Value::from(user_names.join(", ")));
        vars.insert(String::from("recipient_id"), Value::from(id));
        vars.insert(String::from("email_id"), Value::from(email_id.clone()));

        message.vars = Some(vars);
        message.use_mj_template_language = Some(true);
        message.mj_template_id = Some(6073398);
        message.push_recipient(Recipient::new(email_address));

        emails.push(message);
    }

    return Ok(Batch { messages: emails });
}

struct Batch {
    pub messages: Vec<Message>,
}

impl Payload for Batch {
    fn to_json(&self) -> String {
        return format!(
            "{{ \"Messages\": {} }}",
            to_string(&self.messages).unwrap()
        );
    }
}
