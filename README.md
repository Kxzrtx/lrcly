<div align="center">

# 🎵 lrcly

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://kernel.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)

Live synchronized lyrics for your currently playing music, right in the terminal.

</div>

---

## ✨ Features

| MPRIS support | LRCLIB lyrics | Live synchronization | Automatic song switching | Fullscreen terminal UI | Multiple display modes |

---

## 🚀 Installation

### 1. Clone the repository

```bash
git clone https://github.com/YOUR_USERNAME/lrcly.git
cd lrcly
````

### 2. Build

```bash
cargo build --release
```

### 3. Run

```bash
cargo run -- lyrics
```

Or run the compiled binary:

```bash
./target/release/lrcly lyrics
```

Make sure a music player with MPRIS support is currently playing.

---

## 🖥️ Display Modes

### Lyrics

```bash
lrcly lyrics
```

Displays only the current synchronized lyric in fullscreen.

### Info

```bash
lrcly info
```

Displays the artist, title, album and current lyric.

### Time

```bash
lrcly time
```

Displays the current lyric and playback time.

### Full

```bash
lrcly full
```

Displays all available information.

See all options with:

```bash
lrcly --help
```

---

## 📦 Requirements

* Linux
* Rust
* A music player with MPRIS support
* Internet connection for lyric lookup

---

## 🌍 Language Support

The large-text renderer currently supports Latin, Polish and Cyrillic characters.

More Unicode language support is planned.

---

## 🗺️ Roadmap

* [x] MPRIS song detection
* [x] LRCLIB integration
* [x] Synchronized lyrics
* [x] Automatic song switching
* [x] Fullscreen terminal mode
* [x] Multiple display modes
* [x] No-lyrics fallback
* [ ] Improved multilingual rendering
* [ ] Better lyric wrapping
* [ ] Configuration file
* [ ] Custom themes
* [ ] Playback controls
* [ ] Lyrics caching
* [ ] Additional lyrics providers


