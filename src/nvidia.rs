use crate::alldone_reboot;
use crate::mainmenu;
use std::io;
use clearscreen::clear;
use std::process::{Command, Stdio};


pub fn nvidia() {
    let _ = clear();
    println!(r"Select your card:
        1. Maxwell and Pascal (from GTX745 to GTX10-series)
        2. Turing and newer (GTX16-series and newer)
        3. Back to Main Menu");
    let choice: i32;
    loop {
        let mut choice1 = String::new();
        io::stdin()
            .read_line(&mut choice1)
            .expect("Something went wrong, please try again");
        let _choice1: i32 = match choice1.trim().parse() {
            Ok(num) => {
                choice = num;
                break
            }
            Err(_) => continue,
        };
    }
    if choice == 1 {
        determine_kernel();
        closed();
        //determine_kernel();
    }
    else if choice == 2 {
        determine_kernel();
        open();
    }
    else if choice == 3 {
        mainmenu();
    }
    else {
        println!("Select a valid option!");
        nvidia();
    }
}

fn closed() {
    let _closd = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo pacman -Syu --needed nvidia-dkms nvidia-utils nvidia-settings lib32-nvidia-utils && sudo mkinitcpio -P"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to run command!");
    alldone_reboot();
}

fn open() {
    let _opn = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo pacman -Syu --needed nvidia-open-dkms nvidia-utils nvidia-settings lib32-nvidia-utils && sudo mkinitcpio -P"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to run command!");
    alldone_reboot();
}

fn determine_kernel() {
    let header_script = r#"
        for pkg in $(pacman -Q | awk '{print $1}' | grep '^linux'); do
            if pacman -Si "$pkg-headers" &>/dev/null; then
                sudo pacman -Sy --needed "$pkg-headers"
            fi
        done
        "#;
    let _kernel = Command::new("sh")
        .arg("-c")
        .arg(header_script)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to run command!");
}
