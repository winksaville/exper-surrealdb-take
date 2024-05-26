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
    println!("tony: {:?}", tony);

    // Get the Tony Tiger table and id
    let tony_tb = &tony[0].id.tb;
    let tony_id = tony[0].id.id.to_string();
    println!("tony_tb: {tony_tb}, tony.id: {tony_id}");

    // Select Tony Tiger using the id
    let tony_person_by_id: Option<Person> = db.select((tony_tb, tony_id)).await?;
    assert!(tony_person_by_id.is_some(), "Expected Some");
    dbg!(tony_person_by_id);

    // Query all people specifing all the fields individually
    let surql = "SELECT name, age, is_active FROM person";
    let mut result = db.query(surql).await?;
    let people_take0: Vec<Person> = result.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(people_take0);

    // Query all people using the wildcard
    let surql = "SELECT * FROM person";
    let mut result = db.query(surql).await?;
    let people_take0: Vec<Person> = result.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(people_take0);

    // Query Tony Tiger using his the name
    let surql = "SELECT * FROM person WHERE name = 'Tony Tiger'";
    let mut result = db .query(surql).await?;
    let tony_person: Vec<Person> = result.take(0)?;
    assert!(tony_person.len() == 1, "Expected 1 elements");
    dbg!(tony_person);

    Ok(())
}
