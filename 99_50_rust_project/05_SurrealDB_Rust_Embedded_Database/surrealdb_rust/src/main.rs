use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Error, Surreal};

#[derive(Serialize, Deserialize)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

// Pro tip: Replace String with Cow<'static, str> to
// avoid unnecessary heap allocations when inserting
#[derive(Serialize, Deserialize)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

// Install at https://surrealdb.com/install
// and use `surreal start --user root --pass root`
// to start a working database to take the following queries

// See the results via `surreal sql --ns namespace --db database --pretty`
// or https://surrealist.app/
// followed by the query `SELECT * FROM person;`

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<Ws>("localhost:7999").await?;
    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root1",
        password: "root1",
    })
    .await?;
    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;
    // Create a new person with a random ID
    let created: Vec<Person> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: true,
        })
        .await?
        .expect("created error");
    // Create a new person with a specific ID
    let created: Option<Person> = db
        .create(("person", "jaime"))
        .content(Person {
            title: "Founder & COO".into(),
            name: Name {
                first: "Jaime".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: false,
        })
        .await?;
    // Update a person record with a specific ID
    let updated: Option<Person> = db
        .update(("person", "jaime"))
        .merge(json!({"marketing": true}))
        .await?;
    // Select all people records
    let people: Vec<Person> = db.select("person").await?;
    // Perform a custom advanced query
    let query = r#"
        SELECT marketing, count()
        FROM type::table($table)
        GROUP BY marketing
    "#;
    let groups = db.query(query).bind(("table", "person")).await?;
    Ok(())
}
