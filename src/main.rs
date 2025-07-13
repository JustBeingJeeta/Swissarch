mod nvidia;
mod aurhelpers;
mod repos;
use aurhelpers::yay;
use aurhelpers::paru;
use repos::chaoticaur;
use repos::cachy;
use nvidia::nvidia;
use std::io;
use clearscreen::clear;
use std::process::{Command, Stdio};
use system_shutdown::force_reboot;
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
    5. Install Nvidia drivers
    6. Quit");
    get_user_input();
}

fn get_user_input() {
    let user_choice: i32;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong, please try again");
        let _input: i32 = match input.trim().parse() {
            Ok(num) => {
                user_choice = num;
                break;
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
    actions(user_choice)
}
fn actions(user_choice: i32) {
    if user_choice == 1 {
        update();
    }
    else if user_choice == 2 {
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
                    break;
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
    else if user_choice == 3 {
               clear().unwrap();
        println!(r"Choose an custom repo to install:
        1. CachyOS
        2. Chaotic-AUR
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
    else if user_choice == 4 {
        println!(r"WARNING! This may break some packages, please examine the list before entering 'Y'");
        let _orphans = Command::new("sh")
            .arg("-c")
            .arg(format!("pacman -Qtdq | sudo pacman -Rns"))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run command!");
        alldone();
    }
    else if user_choice == 5 {
        nvidia();
    }
    else {
        std::process::exit(0);
    }
}

pub fn update() {
    let _update = Command::new("sudo")
            .args(["pacman", "-Syu"])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run command!");
    alldone()
}

pub fn alldone() {
    println!(r"All done! Enter your next option:
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

pub fn alldone_reboot() {
    println!(r"All done! Enter your next option:
        1. Reboot the system (RECOMMENDED)
        2. Main Menu
        3. Exit");
    let mut _choice: i32;
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Something went wrong, please try again");
        let _choice: i32 = match choice.trim().parse() {
            Ok(1) => {
                let _ = force_reboot();
                break;
            }
            Ok(2) => {
                mainmenu();
                break;
            }
            Ok(_) => {
                std::process::exit(0);
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
                }
        };
    }
}
