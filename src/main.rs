mod anonymiser;
mod db;
mod hash;
mod crypto;

use crate::anonymiser::anonymise;
use crate::db::{save_database_to_file, Teacher, DATABASE};
use crate::hash::{hash_password, verify_hash};
use lazy_static::lazy_static;
use read_input::prelude::*;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::OpenOptions;
use dotenv::dotenv;

// used to store keep track of the entity username because username is not stored in the struct
struct Connected<T> {
    username: String,
    data: T,
}

// create lazy hash for timing attacks
lazy_static! {
    static ref FAKE_HASH: String = hash_password("Dodo_fait_dodo".to_string());
}

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(teacher: &mut Option<Connected<Teacher>>) {
    log::trace!("menu()");
    match teacher {
        Some(teacher) => teacher_action(teacher),
        None => student_action(teacher),
    }
}

fn student_action(teacher: &mut Option<Connected<Teacher>>) {
    log::trace!("student_action()");
    println!("*****\n1: See your grades\n2: Teachers' menu\n3: About\n0: Quit");
    let choice = input().inside(0..=3).msg("Enter Your choice: ").get();
    match choice {
        3 => about(),
        1 => show_grades("Enter your name", false),
        2 => become_teacher(teacher),
        0 => quit(),
        _ => println!("impossible choice, try again"),
    }
}

fn teacher_action(teacher: &Connected<Teacher>) {
    log::trace!("teacher_action({:?})", anonymise(&teacher.username));
    println!("***** Welcome {} *****", teacher.data.name);
    println!("*****\n1: See grades of student\n2: Enter grades\n3 About\n0: Quit");
    let choice = input().inside(0..=2).msg("Enter Your choice: ").get();
    match choice {
        1 => show_grades(
            "Enter the name of the user of which you want to see the grades:",
            true,
        ),
        2 => enter_grade(teacher),
        0 => quit(),
        _ => println!("impossible choice, try again"),
    }
}

fn show_grades(message: &str, is_teacher: bool) {
    log::trace!("show_grades({:?}, {:?})", message, is_teacher);
    println!("{}", message);
    let name: String = input().get();
    let students = DATABASE.students.lock().unwrap();
    let student = students.get(&name);

    // if not teacher ask for password
    if !is_teacher {
        let password: String = rpassword::prompt_password("Enter your password: ").unwrap();
        let valid = match student {
            Some(student) => {
                if verify_hash(password, student.password.clone()) {
                    log::info!("Student({}) logged in successfully", anonymise(&name));
                    true
                } else {
                    log::warn!("Student({}) failed to log in", anonymise(&name));
                    false
                }
            }
            None => {
                verify_hash(password, FAKE_HASH.clone());
                log::warn!("Someone tried to log in as Student with username {}", name);
                false
            }
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

fn become_teacher(teacher: &mut Option<Connected<Teacher>>) {
    log::trace!("become_teacher()");
    let username: String = input::<String>().msg("Enter your username: ").get();
    let password: String = rpassword::prompt_password("Enter your password: ").unwrap();

    let teachers = DATABASE.teachers.lock().unwrap();
    *teacher = match teachers.get(&username) {
        Some(teacher) => {
            if verify_hash(password, teacher.password.clone()) {
                log::info!("Teacher({}) logged in successfully", anonymise(&username));
                Some(Connected {
                    username,
                    data: teacher.clone(),
                })
            } else {
                log::warn!("Teacher({}) failed to log in", anonymise(&username));
                None
            }
        }
        None => {
            verify_hash(password, FAKE_HASH.clone());
            log::warn!(
                "Someone tried to log in as Teacher with username {}",
                username
            );
            None
        }
    };

    if teacher.is_none() {
        println!("Cannot connect to teacher menu");
    }
}

fn enter_grade(teacher: &Connected<Teacher>) {
    log::trace!("enter_grade({:?})", anonymise(&teacher.username));
    println!("What is the name of the student?");
    let name: String = input().get();
    println!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    let mut students = DATABASE.students.lock().unwrap();
    match students.get_mut(&name) {
        Some(v) => {
            v.grades.push(grade);
            log::info!(
                "Teacher({}) added a grade to student({})",
                anonymise(&teacher.username),
                anonymise(&name)
            );
            println!("Grade added");
        }
        None => {
            log::warn!(
                "Teacher({}) tried to add a grade to student({}) but the student does not exist",
                anonymise(&teacher.username),
                name
            );
            println!("User {} does not exist", name);
            return;
        }
    };
}

fn about() {
    log::trace!("about()");
    println!("KING is a student management system");
    println!("Ameliorate by Maxime Scharwath");
}

fn quit() {
    save_database_to_file();
    std::process::exit(0);
}

fn main() {
    dotenv().ok();
    WriteLogger::init(
        LevelFilter::Trace,
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
