use rusqlite::{Connection, Error, Result};

use crate::User;

pub fn save(user: User) -> Result<(), (Connection, Error)> {
    let path = "users.db";
    let conn = Connection::open(path).unwrap();
    let result = conn.execute(
        "INSERT INTO users (name, email, password) VALUES (?1, ?2, ?3)",
        [user.name, user.email, user.password],
    );

    match result {
        Ok(_) => println!("User saved successfully"),
        Err(err) => println!("Error: {}", err),
    }

    conn.close()?;

    Ok(())
}