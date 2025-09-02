# Control
- The player moves up/down/left/right via `WASD`
- The player attacks via arrow keys

# Technical Requirements
1. The game only works and tested on Linux (WSL may work)
2. You must use a terminal emulator that supports progressive keyboard enhancement
  (Usually the [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) which most modern terminal supports it).<br/>
  List of supported terminals:
    - [alacritty](https://alacritty.org/) (Windows, macOS, Linux)
    - [WezTerm](https://wezterm.org/) (Windows, macOS, Linux)
      - [And configuration](https://wezterm.org/config/lua/config/enable_kitty_keyboard.html)
    - [rio](https://rioterm.com/) (Windows, macOS, Linux)
    - [kitty](https://sw.kovidgoyal.net/kitty/) (macOS, Linux)
    - [ghostty](https://ghostty.org/download) (macOS, Linux)
    - [foot](https://wiki.archlinux.org/title/Foot) (Wayland Linux)
    - [iTerm2](https://iterm2.com/) (macOS)
3. The game need *at least* 63x30 cells to render properly otherwise some UI elements may not be visible.

# Installation

## Precompiled Binary
1. Go to github.com/peach-on-the-way/rust-minigame/releases/latest
2. Download the binary that matches your operating system
  - `rust-minigame-x86_64-linux` if you're on Linux.
3. Run it inside a terminal!

## Manual
1. Dowloading the project
  ```
  git clone https://github.com/peach-on-the-way/rust-minigame
  ```
2. Building the project
  ```
  cd rust-minigame
  cargo build
  ```
3. Running the game
  ```
  cargo run
  ```

