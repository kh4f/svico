# 🖼️ svico

<a href='https://crates.io/crates/svico'><img src="https://img.shields.io/crates/v/svico.svg?style=flat-square&label=crates.io&labelColor=b94f16&color=d6c8ba" alt="crates.io"></a>&nbsp;
<a href="https://crates.io/crates/svico"><img src="https://img.shields.io/crates/d/svico.svg?style=flat-square&label=downloads&labelColor=b94f16&color=d6c8ba" alt="downloads"/></a>&nbsp;
<a href="https://docs.rs/svico"><img src="https://img.shields.io/docsrs/svico?style=flat-square&label=docs&labelColor=b94f16&color=d6c8ba" alt="docs"></a>

An SVG to ICO converter optimized for the **smallest lossless output**

`svico` renders the SVG into PNG layers at each requested size, losslessly compresses them with `oxipng`, and assembles them into a single `.ico` file.

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