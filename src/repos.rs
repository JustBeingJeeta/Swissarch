use crate::mainmenu;
use std::io;
use std::process::{Command, Stdio};

pub fn cachy() {
    let cachy1 = Command::new("sh")
        .arg("-c")
        .arg(format!("curl -O https://mirror.cachyos.org/cachyos-repo.tar.xz && tar xvf cachyos-repo.tar.xz && cd cachyos-repo && sudo ./cachyos-repo.sh"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Error while installing CachyOS repos");
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

pub fn chaoticaur() {
 let cachy1 = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com && sudo pacman-key --lsign-key 3056513887B78AEB && sudo pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' && sudo pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Error while installing Chaotic-AUR");
    println!(r"Add the following entry to /etc/pacman.conf and update afterwards:

[chaotic-aur]
Include = /etc/pacman.d/chaotic-mirrorlist");
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

