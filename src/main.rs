mod aurhelpers;
mod repos;
use aurhelpers::yay;
use aurhelpers::paru;
//use::aurhelpers::finished;
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
    3. Install custom repos");
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
    }
    else if usrchoice == 2 {
        clear().unwrap();
        println!(r"Choose an aur helper to install:
        1. yay
        2. paru");
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
            //finished(usrchoice: i32);
            //main();
        }
        else if aurhelper == 2 {
            paru();
            //finished(usrchoice: i32);
        }
        else {
            println!("Enter a valid option!")
        }
    }
    else if usrchoice == 3 {
               clear().unwrap();
        println!(r"Choose an custom repo to install:
        1. CachyOS
        2. Chaotic aur (WON'T ADD THE REPO AUTOMATICALLY, The entry needs to be entered manually)");
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

        
    }
    else {
        main();
    }
}

