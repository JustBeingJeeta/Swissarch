# Swissarch

Swissarch is a CLI tool to manage various aspects of an arch(-based) installation, currently in Alpha state. Swissarch allows you to customize your arch installation modularly with what you want, but with automated steps and a menu-centric environment.

## Current rough spots:
- Pretty raw UX
- No support for flatpak and snap installation
- No automated installation of the program itself

## What it currently does
At its stage, Swissarch allows you to:
- Install yay and/or paru 
- Install CachyOS repos and/or Chaotic-AUR 
- Do a System update
- Install Nvidia drivers automatically (both closed and open modules)
- Remove Orphaned packages

## How to test on your machine

Just run this command, remember not to run any script you find on the internet without reviewing it! 

```
curl -sL https://raw.githubusercontent.com/JustBeingJeeta/Swissarch/master/install.sh | bash
```

### This project is still in early development, of course i plan on adding more features during time, but feel free to inspect or contribute to the code. Any advice on possible additions is gladly accepted

Also, this is my first more serious project, so please be kind :)
