use mysql::*;
use mysql::prelude::*;

fn main() -> Result<()> {
    let url = "mysql://my_user:morhano11@localhost:3306/my_database";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(100) NOT NULL
        )"
    )?;

    conn.exec_drop(
        r"INSERT INTO users (name) VALUES (:name)",
        params! {
            "name" => "Moses",
        }
    )?;

    let selected_users: Vec<String> = conn.query("SELECT name FROM users")?;
    for name in selected_users {
        println!("Name: {}", name);
    }

    Ok(())
}
