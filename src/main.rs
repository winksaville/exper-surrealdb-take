use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i64,
    is_active: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create Tobie Hitchcock
    let tobie: Vec<Record> = db
        .create("person")
        .content(Person {
            name: "Tobie Hitchcock".to_string(),
            age: 30,
            is_active: true,
        })
        .await?;
    dbg!(tobie);

    // Create Tony Tiger
    let tony: Vec<Record> = db
        .create("person")
        .content(Person {
            name: "Tony Tiger".to_string(),
            age: 50,
            is_active: false,
        })
        .await?;
    dbg!(tony);

    // Select all people, it seems you must specify the fields individually
    // in the select statement to serialize into a struct.
    let sql = "SELECT name, age, is_active FROM person";
    let mut result = db.query(sql).await?;
    let people_take0: Vec<Person> = result.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(people_take0);

    Ok(())
}
