use lazy_static::{__Deref, lazy_static};
use read_input::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::collections::{HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use sha2::{Sha256};
use sha2::Digest;


const STUDENTS_DATABASE_FILE: &str = "db.json";
const TEACHERS_DATABASE_FILE: &str = "teachers_db.json";

// create student struct
#[derive(Debug, Serialize, Deserialize)]
struct Student {
    grades: Vec<f32>,
    password: String,
}

lazy_static! {
    static ref STUDENT_DATABASE: Mutex<HashMap<String, Student>> = {
        log::info!("Loading student database");
        let map = read_database_from_file(STUDENTS_DATABASE_FILE).unwrap_or(HashMap::new());
        Mutex::new(map)
    };
    static ref TEACHER_DATABASE: HashMap<String, String> = {
        log::info!("Loading teacher database");
        read_teachers_from_file(TEACHERS_DATABASE_FILE).unwrap_or(HashMap::new())
    };
}

fn sha256_hash(s: String) -> String {
    let hash = Sha256::digest(s);
    format!("{:X}", hash)
}

fn read_database_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, Student>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}

fn read_teachers_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(teacher: &mut Option<String>) {
    match *teacher {
        Some(_) => teacher_action(),
        None => student_action(teacher)
    }
}

fn student_action(teacher: &mut Option<String>){
    println!("*****\n1: See your grades\n2: Teachers' menu\n3: About\n0: Quit");
    let choice = input().inside(0..=2).msg("Enter Your choice: ").get();
    match choice {
        1 => show_grades("Enter your name", false),
        2 => become_teacher(teacher),
        0 => quit(),
        _ => panic!("impossible choice"),
    }
}

fn teacher_action() {
    println!("*****\n1: See grades of student\n2: Enter grades\n3 About\n0: Quit");
    let choice = input().inside(0..=2).msg("Enter Your choice: ").get();
    match choice {
        1 => show_grades("Enter the name of the user of which you want to see the grades:", true),
        2 => enter_grade(),
        0 => quit(),
        _ => panic!("impossible choice"),
    }
}

fn show_grades(message: &str, is_teacher: bool) {
    println!("{}", message);
    let name: String = input().get();
    let db = STUDENT_DATABASE.lock().unwrap();
    let student = db.get(&name);
    // if not teacher ask for password
    if !is_teacher {
        let argon2 = Argon2::default();
        let password: String = input().msg("Enter your password: ").get();
        let valid = match student {
            Some(student) => {
                match  PasswordHash::new(student.password.as_str()) {
                    Ok(hash) => {
                        if argon2.verify_password(password.as_bytes(), &hash).is_ok() {
                            log::info!("{} logged in successfully", name);
                            true
                        } else {
                            log::error!("{} failed to log in", name);
                            false
                        }
                    },
                    _ => false
                }
            }
            None => false
        };
        if !valid {
            println!("Cannot show grades.");
            return;
        }
    }

    match student {
        Some(student) => {
            println!("Here are the grades of user {}", name);
            println!("{:?}", student.grades);
            println!(
                "The average is {}",
                (student.grades.iter().sum::<f32>()) / ((*student).grades.len() as f32)
            );
        }
        None => {
            println!("User {} does not exist", name);
        }
    };
}

fn become_teacher(teacher: &mut Option<String>) {
    let username: String = input::<String>().msg("Enter your username: ").get();
    let password: String = input().msg("Enter your password: ").get();

    let argon2 = Argon2::default();
    *teacher = match TEACHER_DATABASE.get(&username) {
          Some(hash) => {
              match  PasswordHash::new(hash) {
                  Ok(hash) => {
                      if argon2.verify_password(password.as_bytes(), &hash).is_ok() {
                          log::info!("{} logged in successfully", sha256_hash(username.clone()));
                          Some(username)
                      } else {
                          log::error!("{} failed to log in", sha256_hash(username.clone()));
                          None
                      }
                  },
                  _ => None
              }
            }
            None => {
                log::error!("{} failed to log in", sha256_hash(username.clone()));
                None
            }
    }
}

fn enter_grade() {
    println!("What is the name of the student?");
    let name: String = input().get();
    println!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    let mut map = STUDENT_DATABASE.lock().unwrap();
    match map.get_mut(&name) {
        Some(v) => v.grades.push(grade),
        None => {
            map.insert(name, Student {
                grades: vec![grade],
                password: String::new(),
            });
        }
    };
}

fn quit() {
    println!("Saving database!");
    let map = STUDENT_DATABASE.lock().unwrap();
    let file = File::create(STUDENTS_DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &*map).unwrap();
    std::process::exit(0);
}

fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("king.log")
            .unwrap(),
    )
    .unwrap();
    welcome();
    // Option teacher name
    let mut teacher = None;
    loop {
        menu(&mut teacher);
    }
}
