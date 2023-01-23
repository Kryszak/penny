![Build Status](https://github.com/Kryszak/penny/actions/workflows/test.yml/badge.svg)
# Penny
Linux Tui MP3 player

This project was created as a personal goal to improve my Rust skills. For now, there are no plans for any heavy work on new features or improvements. 

## Why Penny?
Inspiration for the name comes from the [character](https://bulbapedia.bulbagarden.net/wiki/Penny) which battle theme is good and feels misplaced in the same time. 

## Dev dependencies
For development on Linux OS, alsa development libraries needs to be installed.

### Fedora
```
dnf install alsa-lib-devel
```
### Arch Linux
```
pacman -S alsa-lib
```
### Ubuntu
```
apt install libasound2
```

## Possible new features
- Theme support
- Playlist functionality
- Fetching more song metadata from external API
- D-Bus integration (MPRIS)
- Native notification when playback starts
- Different styles of audio spectrum visualization
