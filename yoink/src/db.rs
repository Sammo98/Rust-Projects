use anyhow::Result;
use std::{
    collections::VecDeque,
    io::{prelude::*, BufReader, Write},
};

pub struct NotReallyADatabase {
    pub db: VecDeque<String>,
}

impl NotReallyADatabase {
    pub fn init() -> Result<Self> {
        let path = shellexpand::tilde("~/db.txt");

        // Open file or create if file does not exist
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path.as_ref())?;

        // Read txt file to a deque and return as db object
        let db = BufReader::new(f)
            .lines()
            .filter_map(|x| x.ok())
            .collect::<VecDeque<String>>();

        Ok(NotReallyADatabase { db })
    }

    pub fn add(&mut self, key: Option<String>, value: Option<String>) {
        match (key, value) {
            // check whether key exists, if not then push to back of deque
            (Some(k), Some(v)) => {
                if self.db.iter().any(|x| x.starts_with(&format!("{k} - "))) {
                    println!("Key already exists with this name.")
                } else {
                    self.db.push_back(format!("{k} - {v}"));
                }
                println!("Your value has now been added to the 'db'.")
            }

            (_, _) => {
                println!("Must supply key and value arguments to add an item");
            }
        }
    }

    pub fn save(self) -> Result<()> {
        // Get path and open file
        let path = shellexpand::tilde("~/db.txt");
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path.as_ref())?;

        // rewrite db
        for item in self.db.into_iter() {
            _ = writeln!(f, "{item}")?;
        }
        Ok(())
    }
}