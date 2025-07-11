use crate::alldone;
use crate::mainmenu;
use std::process::{Command, Stdio};
use std::env;
use std::io;
use which::which;

pub fn paru() {
    if which("paru").is_ok() {
        println!(r"paru is already installed, please enter an option:
        1. Go back to Main Menu
        2. Quit Swissarch");
        let userquit: i8;
        loop {
            let mut userquit1 = String::new();
            io::stdin()
                .read_line(&mut userquit1)
                .expect("Error!");
            match userquit1.trim().parse() {
                Ok(num) => {
                    userquit = num;
                    break;
                }
                Err(_) => {
                    continue;
                }
            };
        }
        if userquit == 1 {
            mainmenu();
        }
        else if userquit == 2 {
            std::process::exit(0);
        }
    }
    else {
        paruin();
    }
}

pub fn yay() {
    if which("yay").is_ok() {
        println!(r"yay is already installed, please enter an option:
        1. Go back to Main Menu
        2. Quit Swissarch");
        let userquit: i8;
        loop {
            let mut userquit1 = String::new();
            io::stdin()
                .read_line(&mut userquit1)
                .expect("Error!");
            match userquit1.trim().parse() {
                Ok(num) => {
                    userquit = num;
                    break;
                }
                Err(_) => {
                    continue;
                }
            };
        }
        if userquit == 1 {
            mainmenu();
        }
        else if userquit == 2 {
            std::process::exit(0);
        }
    }
    else {
        yayin();
    }
}

fn paruin() {
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
    alldone();

}

fn yayin() {
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
    alldone();
}
