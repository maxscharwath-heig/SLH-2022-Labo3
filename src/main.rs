mod anonymiser;
mod db;

use read_input::prelude::*;
use simplelog::{Config, LevelFilter, WriteLogger};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use std::fs::OpenOptions;
use crate::db::{DATABASE, save_database_to_file, Teacher};
use crate::anonymiser::anonymise;

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(teacher: &mut Option<Teacher>) {
    match teacher {
        Some(teacher) => teacher_action(teacher),
        None => student_action(teacher)
    }
}

fn student_action(teacher: &mut Option<Teacher>){
    println!("*****\n1: See your grades\n2: Teachers' menu\n3: About\n0: Quit");
    let choice = input().inside(0..=2).msg("Enter Your choice: ").get();
    match choice {
        1 => show_grades("Enter your name", false),
        2 => become_teacher(teacher),
        0 => quit(),
        _ => panic!("impossible choice"),
    }
}

fn teacher_action(teacher: &Teacher) {
    println!("***** Welcome {} *****", teacher.name);
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
    let students = DATABASE.students.lock().unwrap();
    let student = students.get(&name);
    // if not teacher ask for password
    if !is_teacher {
        let argon2 = Argon2::default();
        let password: String = input().msg("Enter your password: ").get();
        let valid = match student {
            Some(student) => {
                match  PasswordHash::new(student.password.as_str()) {
                    Ok(hash) => {
                        if argon2.verify_password(password.as_bytes(), &hash).is_ok() {
                            log::info!("Student({}) logged in successfully", anonymise(&name));
                            true
                        } else {
                            log::warn!("Student({}) failed to log in", anonymise(&name));
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

fn become_teacher(teacher: &mut Option<Teacher>) {
    let username: String = input::<String>().msg("Enter your username: ").get();
    let password: String = input().msg("Enter your password: ").get();

    let argon2 = Argon2::default();
    let teachers = DATABASE.teachers.lock().unwrap();
    *teacher = match teachers.get(&username) {
          Some(teacher) => {
              match  PasswordHash::new(&*teacher.password) {
                  Ok(hash) => {
                      if argon2.verify_password(password.as_bytes(), &hash).is_ok() {
                          log::info!("Teacher({}) logged in successfully", anonymise(&username));
                          Some(teacher.clone())
                      } else {
                          log::warn!("Teacher({}) failed to log in", anonymise(&username));
                          None
                      }
                  },
                  _ => None
              }
            }
            None => {
                log::warn!("Someone tried to log in with username {}", username);
                None
            }
    }
}

fn enter_grade() {
    println!("What is the name of the student?");
    let name: String = input().get();
    println!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    let mut students = DATABASE.students.lock().unwrap();
    match students.get_mut(&name) {
        Some(v) => v.grades.push(grade),
        None => {
            println!("User {} does not exist", name);
            return;
        }
    };
}

fn quit() {
    save_database_to_file();
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
