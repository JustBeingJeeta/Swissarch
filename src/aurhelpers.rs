use crate::mainmenu;
use crate::stuff;
use std::process::{Command, Stdio};
use std::env;
use std::io;
//use main::mainmenu;
pub fn paru() {
    let home = env::var("HOME").expect("Could not get HOME environment variable");
    let _aurinstall = Command::new("sudo")
        .args(["pacman", "-S", "--needed", "git", "base-devel"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("The deps couldn't be installed!");
    let paru_dir = format!("{}/paru", home);
    let _aurinstall1 = Command::new("git")
        .args(["clone", "https://aur.archlinux.org/paru.git", &paru_dir])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to clone the git!");
    let _aurinstall2 = Command::new("sh")
        .arg("-c")
        .arg(format!("cd \"{}\" && makepkg -si --noconfirm", paru_dir))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("The rest of the installation couldn't be completed!");
    let _cleanup = Command::new("rm")
        .args(["-rf", &paru_dir])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to clean up paru directory!");
        let choice: i32 = 0;
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Something went wrong, please try again");
        let _choice: i32 = match choice.trim().parse() {
            Ok(num) => {
                choice = num;
                break();
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
    if choice == 1 {
        mainmenu();
    }
    else if choice == 2 {
        stuff(0)
    }
}

pub fn yay() {
    let home = env::var("HOME").expect("Error!");
    let _aurinstall = Command::new("sudo")
        .args(["pacman", "-S", "--needed", "git", "base-devel"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("The deps couldn't be installed!");
    let yay_dir = format!("{}/yay", home);
    let _aurinstall1 = Command::new("git")
        .args(["clone", "https://aur.archlinux.org/yay.git", &yay_dir])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to clone the git!");
    let _aurinstall2 = Command::new("sh")
        .arg("-c")
        .arg(format!("cd \"{}\" && makepkg -si --noconfirm", yay_dir))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("The rest of the installation couldn't be completed!");
    let _cleanup = Command::new("rm")
        .args(["-rf", &yay_dir])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to clean up yay directory!");
    println!(r"The installation was successful!
    1. Main Menu
    2. Exit");
    let mut _choice: i32;
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Something went wrong, please try again");
        let _choice: i32 = match choice.trim().parse() {
            Ok(1) => {
                mainmenu();
                break;
            }
            Ok(2) => {
                std::process::exit(0);
            }
            Ok(_) => {
                continue;
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
}
