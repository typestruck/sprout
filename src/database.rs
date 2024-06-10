use postgres::{Client, Error, NoTls};

pub struct User {
    pub email: String,
    pub sender: String,
    pub unread_messages: i32
}

pub fn connect() -> Result<Client, Error> {
    return Client::connect("host=localhost user=merochat dbname=merochat", NoTls);
}

pub fn select_users_with_unread_messages(client: &mut Client) -> Result<Vec<User>, Error> {
    let query = "
        SELECT r.email, s.name, count(1)
        FROM messages m JOIN users s ON m.sender = s.id JOIN users r ON m.recipient = r.id
        WHERE status < 4 AND date >= (utc_now() - INTERVAL '1 hour')
        GROUP BY recipient, sender, r.email, s.name
        ORDER BY recipient
        ";
    let mut users = Vec::new();

    for row in client.query(query, &[])? {
        users.push(User {
            email: row.get(0),
            sender: row.get(1),
            unread_messages: row.get(2)
        })
    }

    return Ok(users);
}