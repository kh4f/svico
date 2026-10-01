# 🖼️ Svico

<a href='https://crates.io/crates/svico'><img alt="version" src="https://img.shields.io/crates/v/svico.svg?style=flat-square&logo=rust&labelColor=EB5200&color=FFE4D6&label=crates.io"></a>&nbsp;
<a href="https://crates.io/crates/svico"><img alt="downloads" src="https://img.shields.io/crates/d/svico.svg?style=flat-square&labelColor=EB5200&color=FFE4D6&label=%F0%9F%93%A5%20downloads"></a>&nbsp;
<a href="https://docs.rs/svico"><img alt="docs" src="https://img.shields.io/docsrs/svico?style=flat-square&labelColor=EB5200&color=FFE4D6&label=%F0%9F%93%8B%20docs"></a>

**Svico** /ˈsvaɪkoʊ/ is an SVG to ICO converter optimized for the **smallest lossless output**.

It renders the SVG into PNG layers at each requested size, losslessly compresses them with [Oxipng](https://crates.io/crates/oxipng), and assembles them into a single `.ico` file.

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
svico = "0.3"
```

```rust
fn main() -> anyhow::Result<()> {
    svico::convert("icon.svg", "icon.ico", &[16, 24, 32, 256])?;
    Ok(())
}
```