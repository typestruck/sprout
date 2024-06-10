use postgres::{Error};

mod database;

fn main() {
    println!("Checking for emails to send...");
    notify_unread_messages().expect("Couldn't rebuild suggestions!");
    println!("Successfully sent emails");
}

fn notify_unread_messages() -> Result<(), Error> {
    let mut client = database::connect()?; //poor man's monad
    let users = database::select_users_with_unread_messages(&mut client)?;

    for u in users {

    }

    return Ok(());
}