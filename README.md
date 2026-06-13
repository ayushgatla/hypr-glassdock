## Hypr-glassdock (cachy-dock)
<img
  src="https://github.com/ayushgatla/hypr-glassdock/blob/main/image.png?raw=true"
  style="border-radius:1rem;width:100%;max-width:100%;"
/>

A lightweight, customizable application dock written in Rust for Linux desktop environments.

Hypr-glassdock provides a clean launcher interface with support for native applications, Flatpak applications, custom icon

## Features

* Fast and lightweight
* Written in Rust
* GTK4-based interface
* Custom CSS styling
* Flatpak application support
* Native application support
* Easy to modify and extend
* Designed for Hyprland and other Wayland compositors

## Applications Included

Current default configuration includes:

* Firefox
* Zen Browser
* Brave Browser
* Nautilus
* Kitty
* Steam
* Spotify
* Discord
* Stremio
* OnlyOffice

Additional applications can be added by editing `src/main.rs`.

---

## Dependencies

### Arch Linux / CachyOS

```bash
sudo pacman -S gtk4
```

### Rust

Install Rust using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Installation

Clone the repository:

```bash
git clone https://github.com/ayushgatla/hypr-glassdock/
cd Hypr-glassdock
```

Build the project:

```bash
cargo build --release
```

Run:

```bash
./target/release/Hypr-glassdock
```

---

## Development

Run in development mode:

```bash
cargo run
```

---

## Hyprland Keybind

Example Hyprland keybind:

```ini
bind = SUPER, B, exec, /home/USER/Hypr-glassdock/target/release/Hypr-glassdock
```

Reload Hyprland:

```bash
hyprctl reload
```

---

## Project Structure

```text
Hypr-glassdock/
├── src/
│   ├── main.rs
│   └── dock.css
├── Cargo.toml
├── Cargo.lock
├── README.md
└── .gitignore
```

---

## Customization

### Adding Applications

Applications can be added using:

```rust
create_app_button(
    "firefox",
    "firefox",
    icon_size,
);
```

### Flatpak Applications

Example:

```rust
std::process::Command::new("flatpak")
    .args(["run", "com.spotify.Client"])
    .spawn();
```

### Styling

All visual customization is handled through:

```text
src/dock.css
```

Modify the CSS file and rebuild the project.

---

## Building Releases

```bash
cargo build --release
```

Release binary:

```text
target/release/Hypr-glassdock
```

---

## Roadmap

* [ ] Layer-shell support
* [ ] Workspace indicators
* [ ] Auto-hide mode
* [ ] Configuration file support
* [ ] Drag-and-drop app ordering
* [ ] Dynamic icon loading
* [ ] Dock magnification effects
* [ ] Multi-monitor support

---

## Contributing

Pull requests, bug reports, and feature suggestions are welcome.

## License

MIT License
