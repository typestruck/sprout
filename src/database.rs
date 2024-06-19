use tokio_postgres::{Error, NoTls};

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
        WHERE status < 4 AND date >= (utc_now() - INTERVAL '1 hour') AND not(r.temporary) AND r.email LIKE '%@%' AND r.receive_email < 2
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
