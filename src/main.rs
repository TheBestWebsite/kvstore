use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("This is something.");
    println!("Key: {} Value: {}", key, value);
    write_database(key, value)
}

struct Database {
    hashmap: HashMap<String, String>,
}

impl Database {
    fn from_disk() -> Database{
        let contents = std::fs::read_to_string("kv.db");
        Database {
            hashmap: HashMap::new(),
        }
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    let contents = format!("{}\t{}", key, value);
    std::fs::write("kv.db", contents)
}
