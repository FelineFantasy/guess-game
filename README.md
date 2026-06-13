# Guess Game 🎮 [![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

A classic "Guess the Number" game in Rust. The computer picks a random number, and you have to guess it in the minimum number of attempts!

## 📋 Table of Contents
- [Description](#description)
- [How to Play](#how-to-play)
- [Installation](#installation)
- [Running](#running)
- [Requirements](#requirements)
- [Author](#author)

## 📝 Description
**Guess Game** is a simple console game where you need to guess a random number from 1 to 100. After each attempt, the game will hint whether your number is higher or lower.

### Features:
- 🎯 **Simplicity** — Intuitive interface
- 🔄 **Unlimited attempts** — Play until you win
- 📊 **Attempt tracking** — Track your efficiency
- 🎲 **Random numbers** — New game every time

## 🎮 How to Play
1. Run the game
2. Enter a number from 1 to 100
3. Get a hint ("higher" or "lower")
4. Repeat until you guess!

## ⚙️ Installation
### Via Git:
```bash
git clone https://github.com/FelineFantasy/guess-game.git
cd guess-game
```

### Add to Cargo.toml:
```toml
[dependencies]
rand = "0.8"
ask_input = "0.1.0"
```

## 🚀 Running
```bash
cargo run
```

## 📋 Requirements
- Rust 1.70+
- Cargo

## 👤 Author
**FelineFantasy**

License: MIT
