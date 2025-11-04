#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, json::Json};
use database::User;
use itertools::Itertools;
use mailjet_rs::common::{Payload, Recipient};
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use serde_json::{to_string, Map, Value};
use std::borrow::Cow;
use std::env;
use tokio_postgres::Error;
use uuid::Uuid;

mod database;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) => {
            if arg == "--unread" {
                unread_emails().await?;
            } else if arg == "--serve" {
                serve().await?
            }
        }
        _ => println!("Invalid option. Try unread or serve"),
    }

    return Ok(());
}

async fn serve() -> Result<(), rocket::Error> {
    let figment = rocket::Config::figment().merge(("port", 4000));

    rocket::custom(figment)
        .mount("/", routes![reset, report, feedback, approve])
        .launch()
        .await?;

    return Ok(());
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Approve<'r>  {
    field: Cow<'r, str>,
    value: Cow<'r, str>,
    user_id: u64,
}

#[post("/approve", data = "<approved>")]
async fn approve(approved : Json<Approve<'_>>) -> Json<()> {
    let client = Client::new(
        SendAPIVersion::V3,
        env::var("MAILJET_PUBLIC_KEY").unwrap().as_str(),
        env::var("MAILJET_PRIVATE_KEY").unwrap().as_str(),
    );
    let emails = build_approve_email(approved.into_inner());

    println!(
        "Mailjet response {:?}",
        client.send(emails).await.expect("Could not send emails!")
    );

    return Json(());
}

fn build_approve_email(approved : Approve) -> Batch {
    let mut message = Message::new(
        "contact@mero.chat",
        "MeroChat",
        Some("Approve profile change on MeroChat".to_string()),
        None,
    );
    let mut vars = Map::new();

    vars.insert(String::from("field"), Value::from(approved.field));
    vars.insert(String::from("value"), Value::from(approved.value));
    vars.insert(String::from("user_id"), Value::from(approved.user_id));
    vars.insert(String::from("email_id"), Value::from(Uuid::new_v4().to_string().clone()));

    message.vars = Some(vars);
    message.use_mj_template_language = Some(true);
    message.mj_template_id = Some(7459535);
    message.push_recipient(Recipient::new("asafe.hai.kai@gmail.com"));
    message.push_recipient(Recipient::new("e@asafe.dev"));

    return Batch { messages: vec![message] };
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Reset<'r>  {
    token: Cow<'r, str>,
    email: Cow<'r, str>,
    user_id: u64,
}

#[post("/reset", data = "<reseter>")]
async fn reset(reseter : Json<Reset<'_>>) -> Json<()> {
    let client = Client::new(
        SendAPIVersion::V3,
        env::var("MAILJET_PUBLIC_KEY").unwrap().as_str(),
        env::var("MAILJET_PRIVATE_KEY").unwrap().as_str(),
    );
    let emails = build_reset_email(reseter.into_inner());

    println!(
        "Mailjet response {:?}",
        client.send(emails).await.expect("Could not send emails!")
    );

    return Json(());
}

fn build_reset_email(reseted : Reset) -> Batch {
    let mut message = Message::new(
        "contact@mero.chat",
        "MeroChat",
        Some("Password reset on MeroChat".to_string()),
        None,
    );
    let mut vars = Map::new();

    vars.insert(String::from("token"), Value::from(reseted.token));
    vars.insert(String::from("recipient_id"), Value::from(reseted.user_id));
    vars.insert(String::from("email_id"), Value::from(Uuid::new_v4().to_string().clone()));

    message.vars = Some(vars);
    message.use_mj_template_language = Some(true);
    message.mj_template_id = Some(7108800);
    message.push_recipient(Recipient::new(&reseted.email));

    return Batch { messages: vec![message] };
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Report<'r>  {
    reason: Cow<'r, str>,
    comment: Cow<'r,  str>,
    reported: u64,
    reporter: u64,
}

#[post("/report", data = "<reported>")]
async fn report(reported : Json<Report<'_>>) -> Json<()> {
    let client = Client::new(
        SendAPIVersion::V3,
        env::var("MAILJET_PUBLIC_KEY").unwrap().as_str(),
        env::var("MAILJET_PRIVATE_KEY").unwrap().as_str(),
    );
    let emails = build_report_email(reported.into_inner());

    println!(
        "Mailjet response {:?}",
        client.send(emails).await.expect("Could not send emails!")
    );

    return Json(());
}

fn build_report_email(reported : Report) -> Batch {
    let mut message = Message::new(
        "contact@mero.chat",
        "MeroChat",
        Some("New report on MeroChat".to_string()),
        None,
    );
    let mut vars = Map::new();

    vars.insert(String::from("reason"), Value::from(reported.reason));
    vars.insert(String::from("comment"), Value::from(reported.comment));
    vars.insert(String::from("reported"), Value::from(reported.reported));
    vars.insert(String::from("reporter"), Value::from(reported.reporter));
    vars.insert(String::from("recipient_id"), Value::from(4));
    vars.insert(String::from("email_id"), Value::from(Uuid::new_v4().to_string().clone()));

    message.vars = Some(vars);
    message.use_mj_template_language = Some(true);
    message.mj_template_id = Some(7112844);
    message.push_recipient(Recipient::new("asafe.hai.kai@gmail.com"));
    message.push_recipient(Recipient::new("e@asafe.dev"));

    return Batch { messages: vec![message] };
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Feedback<'r>  {
    comments: Cow<'r,  str>,
    file : Cow<'r, str>,
    feedbacker: u64,
}

#[post("/feedback", data = "<feedbacked>")]
async fn feedback(feedbacked : Json<Feedback<'_>>) -> Json<()> {
    let client = Client::new(
        SendAPIVersion::V3,
        env::var("MAILJET_PUBLIC_KEY").unwrap().as_str(),
        env::var("MAILJET_PRIVATE_KEY").unwrap().as_str(),
    );
    let emails = build_feedback_email(feedbacked.into_inner());

    println!(
        "Mailjet response {:?}",
        client.send(emails).await.expect("Could not send emails!")
    );

    return Json(());
}

fn build_feedback_email(feedbacked : Feedback) -> Batch {
    let mut message = Message::new(
        "contact@mero.chat",
        "MeroChat",
        Some("New feedback on MeroChat".to_string()),
        None,
    );
    let mut vars = Map::new();

    vars.insert(String::from("comments"), Value::from(feedbacked.comments));
    vars.insert(String::from("feedbacker"), Value::from(feedbacked.feedbacker));
    vars.insert(String::from("file"), Value::from(feedbacked.file));
    vars.insert(String::from("recipient_id"), Value::from(4));
    vars.insert(String::from("email_id"), Value::from(Uuid::new_v4().to_string().clone()));

    message.vars = Some(vars);
    message.use_mj_template_language = Some(true);
    message.mj_template_id = Some(7113012);
    message.push_recipient(Recipient::new("asafe.hai.kai@gmail.com"));
    message.push_recipient(Recipient::new("e@asafe.dev"));

    return Batch { messages: vec![message] };
}

async fn unread_emails() -> Result<(), Error> {
    let client = Client::new(
        SendAPIVersion::V3,
        env::var("MAILJET_PUBLIC_KEY").unwrap().as_str(),
        env::var("MAILJET_PRIVATE_KEY").unwrap().as_str(),
    );
    let emails = build_unread_emails().await?;

    println!("Checking for emails to send...");

    if emails.messages.is_empty() {
        println!("No emails to send");
    } else {
        database::insert_unsubscribe_tokens(&emails.messages).await?;
        println!(
            "Mailjet response {:?}",
            client.send(emails).await.expect("Could not send emails!")
        );
    }

    return Ok(());
}

async fn build_unread_emails() -> Result<Batch, Error> {
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
            None,
        );
        let mut vars = Map::new();

        vars.insert(String::from("unread_count"), Value::from(count));
        vars.insert(
            String::from("user_names"),
            Value::from(user_names.join(", ")),
        );
        vars.insert(String::from("recipient_id"), Value::from(id));
        vars.insert(String::from("email_id"), Value::from(Uuid::new_v4().to_string().clone()));

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
        return format!("{{ \"Messages\": {} }}", to_string(&self.messages).unwrap());
    }
}

