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

    println!(r#"Define the persons table, not strictly necessary as when using `db.create("perons")` if it does not exist it will be created"#);
    let surql = r#"DEFINE DATABASE persons;"#;
    let response = db.query(surql).await?;
    dbg!(response);
    //match response.take(0) {
    //    Ok(result) => dbg!(result),
    //    Err(e) => {
    //        println!("Error: {e}");
    //        return Err(e);
    //    }
    //}
    //println!("DEFINE DATABASE result: {result:?}");

    // Create Tobie Hitchcock
    let tobie: Vec<Record> = db
        .create("persons")
        .content(Person {
            name: "Tobie Hitchcock".to_string(),
            age: 30,
            is_active: true,
        })
        .await?;
    dbg!(tobie);

    // Create Tony Tiger
    let tony: Vec<Record> = db
        .create("persons")
        .content(Person {
            name: "Tony Tiger".to_string(),
            age: 50,
            is_active: false,
        })
        .await?;
    // Don't use dbg! as it will consume var tony
    println!("tony: {tony:?}");

    // Get the Tony Tiger table and id
    let tony_tb = &tony[0].id.tb;
    let tony_id = tony[0].id.id.to_string();
    println!("tony_tb: {tony_tb}, tony.id: {tony_id}");

    println!("Select Tony Tiger using the id");
    let tony_person_by_id: Option<Person> = db.select((tony_tb, tony_id)).await?;
    assert!(tony_person_by_id.is_some(), "Expected Some");
    dbg!(tony_person_by_id);

    println!("Query all people specifing all the fields individually");
    let surql = "SELECT name, age, is_active FROM persons";
    let mut response = db.query(surql).await?;
    let people_take0: Vec<Person> = response.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(people_take0);

    println!("Query all people using the wildcard");
    let surql = "SELECT * FROM persons";
    let mut response = db.query(surql).await?;
    let people_take0: Vec<Person> = response.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(people_take0);

    println!("Query all people using the wildcard and with stats");
    let mut response = db.query(surql).with_stats().await?;
    if let Some((stats, result)) = response.take(0) {
        let execution_time = stats.execution_time;
        println!("Execution time = {execution_time:?}");
        let people_take0: Vec<Person> = result?;
        assert!(people_take0.len() == 2, "Expected 2 elements");
        dbg!(people_take0);
    } else {
        println!("No result");
    }

    println!("Query Tony Tiger using the name");
    let surql = r#"SELECT * FROM persons WHERE name = "Tony Tiger""#;
    let mut response = db .query(surql).await?;
    let tony_person: Vec<Person> = response.take(0)?;
    assert!(tony_person.len() == 1, "Expected 1 elements");
    dbg!(tony_person);

    Ok(())
}
