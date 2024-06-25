use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let mut db = Database::from_disk()?;
    println!("Key: {} Value: {}", key, value);
    let db_borrow_mut = &mut db;
    db.insert(key, value);
    db.flush()?;
    db_borrow_mut.insert(String::new(), String::new());
    // write_database(key, value)
    Ok(())
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
        let mut hashmap: HashMap<String, String> = HashMap::new();
        for line in contents.lines() {
            let mut chunks: std::str::Split<char> = line.split('\t');
            let key: &str = chunks.next().unwrap();
            let value: &str = chunks.next().unwrap();
            hashmap.insert(key.to_string(), value.to_string());
        }
        Ok(Database {
            hashmap: hashmap,
        })
    }
    fn insert(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }
    fn flush (&self) -> std::io::Result<()> {
        let contents: String = todo!("Format the keys and values as a string");
        std::fs::write("kv.db", contents)
    }
}


//fn function() {
  //  let _s3 = foo();
//}

//fn foo() -> String{
  //  let s = String::from("Hello, world");
  //  let s2 = s;
  //  println!("{}", s2);
  //  s2
//}