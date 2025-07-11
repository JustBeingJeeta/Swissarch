# Swissarch

Swissarch is a CLI tool to manage various aspects of an arch(-based) installation, currently in pre-alpha state. Swissarch allows you to customize your arch installation modularly with what you want, but with automated steps and a menu-centric environment.

## Current rough spots:
- Requires manual steps to install Chaotic-AUR
- Pretty raw UX
- No support for flatpak and snap installation
- No automated installation of the program itself

## What it currently does
At its stage, Swissarch allows you to:
- Install yay and/or paru automatically
- Install CachyOS repos and (partially) Chaotic-AUR
- System update
- Remove Orphaned packages

## How to test on your machine

1. Clone the repository with:
```
git clone https://github.com/JustBeingJeeta/Swissarch && cd Swissarch
```

2. Make the binary executable:
```
chmod +x swissarch
```

3. Open the binary:
```
./swissarch
```

### This project is still in early development, of course i plan on adding more features during time, but feel free to inspect or contribute to the code. Also, any advice on possible additions is gladly accepted

Also, this is my first more serious project, so please be kind :)
