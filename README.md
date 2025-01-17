# Loungy

![Loungy](./img/preview.webp)

## What

Loungy is a WIP launcher in the vein of Spotlight, Alfred, Raycast.

## Why

Mostly as a learning/hobby project. It isn't (yet) intended to be used outside of experimentation.
I got the idea while writing a Raycast plugin and getting frustrated with their limitations.

## How

Initially I wrote it using Tauri as I am familiar with web technoligies, but quickly got fed up with moving info from rust side to the webview. Around the same time the awesome folks from [Zed](https://zed.dev/) open sourced [GPUI](https://www.gpui.rs/), which is a Rust based GPU accelerated application framework. It instantly intrigued me due to its Tailwind CSS inspiration.

### Credits

Loungy wouldn't be possible without the awesome open source ecosystem:

- [GPUI](https://www.gpui.rs/) : The lovely framework
- [Numbat](https://numbat.dev/) : Used as the calculator
- [Lucide](https://lucide.dev/) : Amazing open source SVG icon-set
- [Catppuccin](https://github.com/catppuccin) : The theme that makes everything look good
- [swift-rs](https://github.com/Brendonovich/swift-rs) : For providing a way to interface with accessibility APIs and other MacOS native stuff that I wouldn't know how to do with pure Rust
- [nucleo](https://github.com/helix-editor/nucleo) : Fuzzy searcher implemented by the team of my favorite modal editor [Helix](https://github.com/helix-editor/helix)

## Features

- [x] Launching apps
- [x] Calculator (including unit/currency conversions, thanks to [Numbat](https://numbat.dev/))
- [x] Task manager (killing processes)
- [x] MacOS menu search

These features exist in the old Tauri based app and will be ported soon:

- [ ] Bitwarden password manager
- [ ] Tailscale peer list
- [ ] Matrix Chat client

## Development

To run simply:

```
cargo run dev
```
