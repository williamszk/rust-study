// project.rs is a generic name, a name that I don't know which other to use
use std::error;

#[derive(Debug)]
pub struct ExperimentStruct {
    pub name: String,
}

pub fn _test_read_csv() {
    use serde::Deserialize;
    use std::error::Error;

    #[derive(Debug, Deserialize)]
    struct CsvRecord {
        name: String,
        department: String,
        birthday_month: String,
    }

    fn read_csv() -> Result<String, Box<dyn Error>> {
        // fn read_csv() {
        let reader = csv::Reader::from_path("data/sample.csv");
        for result in reader?.deserialize() {
            // Automatic deserialization
            let record: CsvRecord = result?;
            println!(
                "Name: {} | Department: {} | Birth Month: {}",
                record.name, record.department, record.birthday_month,
            );
        }
        Ok(String::from("csv read successfully!"))
    }
    let result_read = read_csv();
    match result_read {
        Ok(message) => println!("{}", message),
        Err(error) => println!("{}", error),
    };
}

pub async fn _test_request_for_json() -> Result<(), Box<dyn error::Error>> {
    let resp = reqwest::get("http://dummyjson.com/comments")
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", resp);

    // while let Some(entry) = resp.get("comments") {
    //     println!("{:#?}", entry);
    // }

    Ok(())
}

// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn _foo() -> u8 {
    5
}

// Example of us of async await and Future
async fn _calling_foo() {
    let res = _foo();

    // in here we do some stuff which takes some network or IO time...
    // we can use this time to process other things

    res.await;
}

pub fn _experiment_with_sqlite() -> rusqlite::Result<()> {
    // https://karimjedda.com/carefully-exploring-rust

    struct Person {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    let conn = rusqlite::Connection::open("people.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person(
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BLOB
        )",
        (), // Empty list of parameters
    )?;

    let p1 = Person {
        id: 0,
        name: String::from("Bob"),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&p1.name, &p1.data),
    )?;

    println!("Successfully insert into sqlite database.");

    Ok(())
}
