use crate::update;
use crate::alldone;
//use crate::mainmenu;
//use std::io;
use std::process::{Command, Stdio};

pub fn cachy() {
    let _cachy1 = Command::new("sh")
        .arg("-c")
        .arg(format!("curl -O https://mirror.cachyos.org/cachyos-repo.tar.xz && tar xvf cachyos-repo.tar.xz && cd cachyos-repo && sudo ./cachyos-repo.sh"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Error while installing CachyOS repos");
    alldone();
    }

pub fn chaoticaur() {
 let _cachy1 = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com && sudo pacman-key --lsign-key 3056513887B78AEB && sudo pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' && sudo pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Error while installing Chaotic-AUR");
    addtopacman();
    update();
    alldone();
    }

fn addtopacman() {
    let _add_to_pacman = Command::new("sh")
    .arg("-c")
    .arg(format!("echo [chaotic-aur] | sudo tee -a /etc/pacman.conf"))
    .output()
    .expect("Oops");
    let _add_to_pacman1 = Command::new("sh")
    .arg("-c")
    .arg(format!("echo Include = /etc/pacman.d/chaotic-mirrorlist | sudo tee -a /etc/pacman.conf"))
    .output()
    .expect("Oops");
}
