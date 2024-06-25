use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let db = Database::from_disk()?;
    db.insert(key, value);
    println!("Key: {} Value: {}", key, value);
    write_database(key, value)
}

struct Database {
    hashmap: HashMap<String, String>,
}

impl Database {
    fn from_disk() -> Result<Database, std::io::Error>{
        let contents = std::fs::read_to_string("kv.db")?;
        // That question mark up there does the same as these three lines of code:
        // let contents = match contents {
        //      Ok(c) => c,
        //      Err(error_value) => return Err(error_value)
        // };
        let mut hashmap = HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.split('\t');
            let key = chunks.next().unwrap();
            let value = chunks.next().unwrap();
            hashmap.insert(key.to_string(), value.to_string());
        }
        Ok(Database {
            hashmap: hashmap,
        })
    }
    fn insert(mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    let contents = format!("{}\t{}", key, value);
    std::fs::write("kv.db", contents)
}

fn foo() {
    let s = String::new();
}