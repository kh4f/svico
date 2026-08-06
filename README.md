# 🖼️ svico

Convert SVG to a tiny, lossless multi-resolution ICO.

## 📥 Install

```bash
cargo install svico
```

## 🕹️ Usage

```bash
Usage: svico <input.svg> [options]

Options:
  -o, --output <path>  Output .ico path [<input>.ico]
  -s, --sizes <list>   Comma-separated sizes in 1..=256 [16,24,32,256]
  -h, --help           Print help

Examples:
  svico icon.svg
  svico icon.svg -o app/favicon.ico
  svico icon.svg -s 64,128,256
```