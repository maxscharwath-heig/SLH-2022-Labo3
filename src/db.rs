use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

const DATABASE_FILE: &str = "db.json";

// create student struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub(crate) grades: Vec<f32>,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Teacher {
    pub(crate) name: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataBase {
    pub students: Mutex<HashMap<String, Student>>,
    pub teachers: Mutex<HashMap<String, Teacher>>,
}

lazy_static! {
    pub static ref DATABASE: DataBase = {
        read_database_from_file(DATABASE_FILE).unwrap_or_else(|_| {
            log::error!("Failed to load database");
            DataBase {
                students: Mutex::new(HashMap::new()),
                teachers: Mutex::new(HashMap::new()),
            }
        })
    };
}

fn read_database_from_file<P: AsRef<Path>>(path: P) -> Result<DataBase, Box<dyn Error>> {
    log::info!("Loading database");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}

pub fn save_database_to_file() {
    log::info!("Saving database");
    let db = &*DATABASE;
    let file = File::create(DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, db).unwrap();
}
