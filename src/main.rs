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

    // Define the persons table with "IF NOT EXISTS" which requires feature "sql2"
    let surql = r#"DEFINE DATABASE IF NOT EXISTS persons;"#;
    println!(r#"Define the persons table the easy way"#);
    let response = db.query(surql).await?;
    println!("Successfully created DB");
    dbg!(&response);

    // Define the persons table again, surprisingly IF NOT EXISTS is not needed
    // to ignore the error.
    let surql = r#"DEFINE DATABASE persons;"#;
    println!(r#"Define the persons table with error handling"#);
    match db.query(surql).await {
        Ok(response) => {
            println!("Successfully created DB");
            dbg!(&response);
        }
        Err(e) => {
            println!("Error: {e}");
            return Err(e);
        }
    }

    // Create Tobie Hitchcock
    let tobie: Vec<Record> = db
        .create("persons")
        .content(Person {
            name: "Tobie Hitchcock".to_string(),
            age: 30,
            is_active: true,
        })
        .await?;
    dbg!(&tobie);

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
    dbg!(&tony);

    println!("Select Tony Tiger using the id");
    let tony_tb = &tony[0].id.tb;
    dbg!(&tony_tb);
    let tony_id = tony[0].id.id.to_string();
    dbg!(&tony_id);
    let tony_person_by_id: Option<Person> = db.select((tony_tb, tony_id)).await?;
    assert!(tony_person_by_id.is_some(), "Expected Some");
    dbg!(&tony_person_by_id);

    println!("Query Tony Tiger using the name");
    let surql = r#"SELECT * FROM persons WHERE name = "Tony Tiger""#;
    let mut response = db.query(surql).await?;
    let tony_person: Vec<Person> = response.take(0)?;
    assert!(tony_person.len() == 1, "Expected 1 elements");
    dbg!(&tony_person);

    println!("Query all people specifing all the fields individually");
    let surql = "SELECT name, age, is_active FROM persons";
    let mut response = db.query(surql).await?;
    let people_take0: Vec<Person> = response.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(&people_take0);

    println!("Query all people using the wildcard");
    let surql = "SELECT * FROM persons";
    let mut response = db.query(surql).await?;
    let people_take0: Vec<Person> = response.take(0)?;
    assert!(people_take0.len() == 2, "Expected 2 elements");
    dbg!(&people_take0);

    println!("Query all people using the wildcard and with stats and complete error handling!");
    let mut response = match db.query(surql).with_stats().await {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {e}");
            return Err(e);
        }
    };

    if let Some((stats, result)) = response.take(0) {
        let execution_time = stats.execution_time;
        println!("Execution time = {execution_time:?}");
        let people_take0: Vec<Person> = match result {
            Ok(result) => result,
            Err(e) => {
                println!("Error: {e}");
                return Err(e);
            }
        };
        assert!(people_take0.len() == 2, "Expected 2 elements");
        dbg!(&people_take0);
    } else {
        println!("No result");
    }

    Ok(())
}
