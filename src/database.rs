use tokio_postgres::{Error, NoTls};
use mailjet_rs::v3::Message;

pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub unread_messages: i64,
}

pub async fn select_users_with_unread_messages() -> Result<Vec<User>, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=merochat dbname=merochat", NoTls).await?;
    let query = "
        SELECT r.id, r.email, s.name, count(1)
        FROM messages m JOIN users s ON m.sender = s.id JOIN users r ON m.recipient = r.id
        WHERE
            status >= 0 AND status < 3 AND
            date >= (utc_now() - INTERVAL '1 day') AND
            not r.temporary AND
            r.email LIKE '%@%' AND
            r.receive_email < 2 AND
            NOT EXISTS (select 1 from histories h where h.recipient = m.recipient AND h.recipient_deleted_to IS NOT NULL AND m.id <= h.recipient_deleted_to)
        GROUP BY recipient, sender, r.email, r.id, s.name
        ORDER BY recipient
        ";
    let mut users = Vec::new();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    for row in client.query(query, &[]).await? {
        users.push(User {
            id: row.get(0),
            email: row.get(1),
            name: row.get(2),
            unread_messages: row.get(3),
        })
    }

    return Ok(users);
}

pub async fn insert_unsubscribe_tokens(messages: &Vec<Message>) -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=merochat dbname=merochat", NoTls).await?;
    let query = "insert into unsubscribe_tokens (unsubscriber, contents) values ";
    let mut rows = Vec::new();

    for m in messages {
        rows.push(format!("({}, '{}')", m.vars.as_ref().unwrap().get("recipient_id").unwrap().as_i64().unwrap(), m.vars.as_ref().unwrap().get("email_id").unwrap().as_str().unwrap()));
    }

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(&format!("{} {}", query, rows.join(", ")), &[]).await?;

    return Ok(());
}
