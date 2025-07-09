mod aurhelpers;
mod repos;
use aurhelpers::yay;
use aurhelpers::paru;
use repos::chaoticaur;
use repos::cachy;
use std::io;
use clearscreen::clear;
use std::process::{Command, Stdio};
fn main() {
    mainmenu()
}
pub fn mainmenu()  {
    clear().unwrap();
    println!(r"
    ┏━┛┃┃┃┛┏━┛┏━┛┏━┃┏━┃┏━┛┃ ┃
    ━━┃┃┃┃┃━━┃━━┃┏━┃┏┏┛┃  ┏━┃
    ━━┛━━┛┛━━┛━━┛┛ ┛┛ ┛━━┛┛ ┛

    ");
    println!("Swiss-arch, an all-in-one tool to manage arch linux");
    println!(r"Select an option:
    1. Update
    2. Install aur helper
    3. Install custom repos
    4. Remove orphans
    5. Quit");
    getusrinput();
}

fn getusrinput() {
    let usrchoice: i32;
    loop {
        let mut userchoice = String::new();
        io::stdin()
            .read_line(&mut userchoice)
            .expect("Something went wrong, please try again");
        let _usrchoice: i32 = match userchoice.trim().parse() {
            Ok(num) => {
                usrchoice = num;
                break();
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
    stuff(usrchoice)
}
fn stuff(usrchoice: i32) {
    if usrchoice == 1 {
        let _update = Command::new("sudo")
            .args(["pacman", "-Syu"])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run command!");
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
    else if usrchoice == 2 {
        clear().unwrap();
        println!(r"Choose an aur helper to install:
        1. Yay
        2. Paru
        3. Back to main menu");
        let aurhelper: i8;
        loop {
            let mut aur = String::new();
            io::stdin()
                .read_line(&mut aur)
                .expect("Something went wrong, try again");
            let _aurhelper: i8 = match aur.trim().parse() {
                Ok(num) => {
                    aurhelper = num;
                    break();
                }
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };
        };
        if aurhelper == 1 {
            yay();
        }
        else if aurhelper == 2 {
            paru();
        }
        else {
            mainmenu();
        }
    }
    else if usrchoice == 3 {
               clear().unwrap();
        println!(r"Choose an custom repo to install:
        1. CachyOS
        2. Chaotic aur (WON'T ADD THE REPO AUTOMATICALLY, The entry needs to be entered manually)
        3. Back to main menu");
        let crepo: i8;
        loop {
            let mut repo = String::new();
            io::stdin()
                .read_line(&mut repo)
                .expect("Something went wrong, try again");
            let _crepo: i8 = match repo.trim().parse() {
                Ok(num) => {
                    crepo = num;
                    break();
                }
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };
        }

        if crepo == 1 {
            cachy();
        }
        else if crepo == 2 {
            chaoticaur();
        }
        else {
            mainmenu();
        }

        
    }
    else if usrchoice == 4 {
        println!(r"WARNING! This may break some packages, please examine the list before entering 'Y'");
        let _orphans = Command::new("sh")
            .arg("-c")
            .arg(format!("pacman -Qtdq | sudo pacman -Rns"))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run command!");
        mainmenu()
    }
    else {
        std::process::exit;
    }
}

