# Swissarch

Swissarch is a CLI tool to manage various aspects of an arch(-based) installation, currently in pre-alpha state. Swissarch allows you to customize your arch installation modularly with what you want, but with automated steps and a menu-centric environment.

## Current rough spots:
- No yay/paru detection
- Requires manual steps to install Chaotic-aur
- Pretty raw UX
- No support for flatpak and snap installation
- No automated installation

## Features to add:

- Complete Chaotic-aur installation.
- Check for yay and paru to see if they were already installed.
- Complete flatpak and snap support.
- Custom meta packages (Gaming, General utils, system utils, etc.)
- Removal of yay or paru

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


