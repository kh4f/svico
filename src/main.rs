use std::{env, error::Error, fs, path::PathBuf};

use resvg::{tiny_skia, usvg};

const DEFAULT_SIZES: &[u32] = &[16, 24, 32, 256];

type AnyResult<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> AnyResult {
    let mut svg_path = None;
    let mut ico_path = None;
    let mut sizes = DEFAULT_SIZES.to_vec();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                let Some(value) = args.next() else {
                    return Err("missing value for -o/--output".into());
                };
                ico_path = Some(PathBuf::from(value));
            }
            "-s" | "--sizes" => {
                let Some(value) = args.next() else {
                    return Err("missing value for -s/--sizes".into());
                };
                sizes = value
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .parse::<u32>()
                            .map_err(|_| format!("invalid size: {s}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}").into()),
            _ => svg_path = Some(arg),
        }
    }

    if sizes.is_empty() || sizes.iter().any(|&s| s == 0 || s > 256) {
        return Err("sizes must be > 0 and < 257".into());
    }

    let Some(svg_path) = svg_path else {
        return Err(
            "usage: svico <input.svg> [-o|--output <output.ico>] [-s|--sizes 16,24,32,256]".into(),
        );
    };
    let ico_path = ico_path.unwrap_or_else(|| PathBuf::from(&svg_path).with_extension("ico"));

    let svg_data = fs::read(&svg_path)?;
    let svg_tree = usvg::Tree::from_data(&svg_data, &usvg::Options::default())?;
    let svg_size = svg_tree.size();

    let png_layers = sizes
        .iter()
        .map(|&size| {
            let png = render_to_png(&svg_tree, &svg_size, size)?;
            println!("Layer {size}×{size}: {} bytes", png.len());
            Ok::<_, Box<dyn Error>>((size, png))
        })
        .collect::<AnyResult<Vec<_>>>()?;

    let ico = build_ico(&png_layers);
    fs::write(&ico_path, &ico)?;
    println!("Icon created: {} ({} bytes)", ico_path.display(), ico.len());
    Ok(())
}

/// Renders the SVG to a `size`×`size` PNG layer.
fn render_to_png(svg_tree: &usvg::Tree, svg_size: &usvg::Size, size: u32) -> AnyResult<Vec<u8>> {
    let mut pixmap = tiny_skia::Pixmap::new(size, size).ok_or("failed to allocate pixmap")?;

    let scale = (size as f32 / svg_size.width()).min(size as f32 / svg_size.height());
    let dx = (size as f32 - svg_size.width() * scale) / 2.0;
    let dy = (size as f32 - svg_size.height() * scale) / 2.0;

    resvg::render(
        svg_tree,
        tiny_skia::Transform::from_scale(scale, scale).post_translate(dx, dy),
        &mut pixmap.as_mut(),
    );

    encode_png(size, size, &unpremultiply(pixmap.data()))
}

/// Builds an ICO container from PNG layers.
///
/// See [the ICO file format spec](https://en.wikipedia.org/wiki/ICO_(file_format)#File_structure).
fn build_ico(png_layers: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let layer_count = png_layers.len() as u16;
    let mut ico = Vec::new();

    ico.extend_from_slice(&0u16.to_le_bytes()); // reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // type: icon
    ico.extend_from_slice(&layer_count.to_le_bytes()); // image count

    // ICONDIRENTRY array
    let mut offset = 6u32 + 16 * layer_count as u32;
    for &(dim, ref buf) in png_layers {
        let wh = if dim == 256 { 0 } else { dim as u8 }; // 0 == 256
        ico.extend_from_slice(&[wh, wh, 0, 0]); // w, h, colors, reserved
        ico.extend_from_slice(&1u16.to_le_bytes()); // planes
        ico.extend_from_slice(&32u16.to_le_bytes()); // bpp
        ico.extend_from_slice(&(buf.len() as u32).to_le_bytes()); // size
        ico.extend_from_slice(&offset.to_le_bytes()); // offset
        offset += buf.len() as u32;
    }

    // PNG blobs
    for (_, buf) in png_layers {
        ico.extend_from_slice(buf);
    }

    ico
}

/// Encodes RGBA to PNG, then losslessly re-compresses with oxipng.
fn encode_png(width: u32, height: u32, rgba: &[u8]) -> AnyResult<Vec<u8>> {
    let mut data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut data, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::High);
        encoder.set_filter(png::Filter::NoFilter);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(rgba)?;
    }

    Ok(oxipng::optimize_from_memory(
        &data,
        &oxipng::Options::max_compression(),
    )?)
}

/// Converts premultiplied alpha back to straight RGBA.
fn unpremultiply(data: &[u8]) -> Vec<u8> {
    let mut out = data.to_vec();

    for px in out.chunks_exact_mut(4) {
        let a = px[3];
        if a > 0 && a < 255 {
            let alpha = a as f32 / 255.0;
            for c in &mut px[..3] {
                *c = (*c as f32 / alpha).clamp(0.0, 255.0).round() as u8;
            }
        }
    }

    out
}
