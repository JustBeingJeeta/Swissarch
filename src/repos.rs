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
}

