use rusqlite::Connection;
use crate::contracts::User;

pub fn query_all_users() -> Vec<User> {
    let path = "users.db";
    let conn = Connection::open(path).unwrap();
    let query = "SELECT * FROM users";
    let mut stmt = conn.prepare(query).unwrap();
    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            password: row.get(3)?,
        })
    }).unwrap();
    
    let mut all_users = Vec::new();
    for user in users {
        all_users.push(user.unwrap());
    }
    all_users
}