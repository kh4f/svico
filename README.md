# 🖼️ svico

An SVG to ICO converter optimized for the **smallest lossless output**

## 💡 How it works

`svico` renders the SVG at each requested size with [`resvg`](https://crates.io/crates/resvg), encodes the raw pixels as PNG with [`png`](https://crates.io/crates/png), losslessly compresses each layer with [`oxipng`](https://crates.io/crates/oxipng), and hand-assembles them into an [ICO container](https://en.wikipedia.org/wiki/ICO_\(file_format\)#File_structure).

## 🕹️ CLI

```bash
cargo install svico
```

<sup>or download a [prebuilt binary](https://github.com/kh4f/svico/releases)</sup>

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

## 🧩 API

```toml
[dependencies]
svico = "0.2"
```

```rust
fn main() -> anyhow::Result<()> {
    svico::convert("icon.svg", "icon.ico", &[16, 24, 32, 256])?;
    Ok(())
}
```