use core::str;
use std::io::{stdin, stdout, Write};
use std::{fs, u64};

use indicatif::ProgressBar;

use crate::visual;

pub fn rename_many_files(dir: &str, rn_to: &str) {
    let cont = user_input("a matching patern");

    let to_process = operation_count(dir, &cont);
    let mut total_edited = 0;

    let pb: ProgressBar = visual::progress_bar(to_process);

    match fs::read_dir(dir) {
        Ok(results) => {
            for result in results {
                match result {
                    Ok(result) => {
                        if result.file_name().to_string_lossy().contains(&cont) {
                            let rep_to: String =
                                result.file_name().to_string_lossy().replace(&cont, rn_to);

                            let result_con = format!("{}{}", dir, rep_to);

                            let _ = fs::rename(result.path().display().to_string(), result_con);

                            total_edited += 1;
                            pb.set_position(total_edited);
                        }
                    }
                    Err(e) => eprintln!("Error {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error {}", e),
    }
    pb.finish_with_message("Complete");
    println!("Total files renamed: {}", to_process);
}

pub fn strip_rename_many_files(dir: &str) {
    let cont = user_input("a matching patern");

    let to_process = operation_count(dir, &cont);
    let mut total_edited = 0;

    let pb: ProgressBar = visual::progress_bar(to_process);

    match fs::read_dir(dir) {
        Ok(results) => {
            for result in results {
                match result {
                    Ok(result) => {
                        if result.file_name().to_string_lossy().contains(&cont) {
                            let rep_to: String =
                                result.file_name().to_string_lossy().replace(&cont, "");

                            let result_con = format!("{}{}", dir, rep_to);

                            let _ = fs::rename(result.path().display().to_string(), result_con);

                            total_edited += 1;
                            pb.set_position(total_edited);
                        }
                    }
                    Err(e) => eprintln!("Error {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error {}", e),
    }
    pb.finish_with_message("Complete");
    println!("Total files renamed: {}", to_process);
}

pub fn delete_many_files(dir: &str) {
    let cont = user_input("a matching patern");

    let to_process = operation_count(dir, &cont);
    let mut total_edited = 0;

    let pb: ProgressBar = visual::progress_bar(to_process);

    match fs::read_dir(dir) {
        Ok(results) => {
            for result in results {
                match result {
                    Ok(result) => {
                        if result.file_name().to_string_lossy().contains(&cont) {
                            let _ = fs::remove_file(result.path().display().to_string());

                            total_edited += 1;
                            pb.set_position(total_edited);
                        }
                    }
                    Err(e) => eprintln!("Error {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error {}", e),
    }
    pb.finish_with_message("Complete");
    println!("Total deleted files : {}", to_process);
}

pub fn delete_many_dirs(dir: &str) {
    let cont = user_input("a matching patern");

    let to_process = operation_count(dir, &cont);
    let mut total_edited = 0;

    let pb: ProgressBar = visual::progress_bar(to_process);

    match fs::read_dir(dir) {
        Ok(results) => {
            for result in results {
                match result {
                    Ok(result) => {
                        if result.file_name().to_string_lossy().contains(&cont) {
                            let _ = fs::remove_dir_all(result.path().display().to_string());

                            total_edited += 1;
                            pb.set_position(total_edited);
                        }
                    }
                    Err(e) => eprintln!("Error {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error {}", e),
    }
    pb.finish_with_message("Complete");
    println!("Total deleted directories: {}", to_process);
}
fn operation_count(dir: &str, cont: &str) -> u64 {
    let mut to_process = 0;
    match fs::read_dir(dir) {
        Ok(results) => {
            for result in results {
                match result {
                    Ok(result) => {
                        if result.path().display().to_string().contains(&cont) {
                            to_process += 1;
                        }
                    }
                    Err(e) => eprintln!("Error {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error {}", e),
    }

    to_process
}

pub fn user_input(fd: &str) -> String {
    let mut inp = String::new();
    print!("Please enter {:?}: ", fd);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut inp)
        .expect("Did not enter a correct string");
    if let Some('\n') = inp.chars().next_back() {
        inp.pop();
    }
    if let Some('\r') = inp.chars().next_back() {
        inp.pop();
    }
    // println!("You typed: {}", inp);
    inp
}
