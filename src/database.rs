use postgres::{Client, Error, NoTls};

pub struct User {
    pub id: u32,
    pub email: String,
    pub name: String,
    pub unread_messages: i32
}

pub fn connect() -> Result<Client, Error> {
    return Client::connect("host=localhost user=merochat dbname=merochat", NoTls);
}

pub fn select_users_with_unread_messages(client: &mut Client) -> Result<Vec<User>, Error> {
    let query = "
        SELECT r.id, r.email, s.name, count(1)
        FROM messages m JOIN users s ON m.sender = s.id JOIN users r ON m.recipient = r.id
        WHERE status < 4 AND date >= (utc_now() - INTERVAL '1 hour') AND not(r.temporary) AND r.email LIKE '%@%' AND r.receive_emails < 2
        GROUP BY recipient, sender, r.email, s.name
        ORDER BY recipient
        ";
    let mut users = Vec::new();

    for row in client.query(query, &[])? {
        users.push(User {
            id : row.get(0),
            email: row.get(1),
            name: row.get(2),
            unread_messages: row.get(3)
        })
    }

    return Ok(users);
}