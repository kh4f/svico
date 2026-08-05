use std::{error::Error, fs};

use resvg::{tiny_skia, usvg};

const SIZES: [u32; 4] = [16, 24, 32, 256];

type AnyResult<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> AnyResult {
    let svg_data = fs::read("icon.svg")?;
    let svg_tree = usvg::Tree::from_data(&svg_data, &usvg::Options::default())?;
    let svg_size = svg_tree.size();

    let png_layers = SIZES
        .iter()
        .map(|&size| {
            let png = render_to_png(&svg_tree, &svg_size, size)?;
            println!("Layer {size}×{size}: {} bytes", png.len());
            Ok::<_, Box<dyn Error>>((size, png))
        })
        .collect::<AnyResult<Vec<_>>>()?;

    let ico = build_ico(&png_layers);
    fs::write("icon.ico", &ico)?;
    println!("Icon created: icon.ico ({} bytes)", ico.len());
    Ok(())
}

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

fn build_ico(png_layers: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let layer_count = png_layers.len() as u16;
    let mut ico = Vec::new();

    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&layer_count.to_le_bytes());

    let mut offset = 6u32 + 16 * layer_count as u32;
    for &(dim, ref buf) in png_layers {
        let wh = if dim == 256 { 0 } else { dim as u8 };
        ico.extend_from_slice(&[wh, wh, 0, 0]);
        ico.extend_from_slice(&1u16.to_le_bytes());
        ico.extend_from_slice(&32u16.to_le_bytes());
        ico.extend_from_slice(&(buf.len() as u32).to_le_bytes());
        ico.extend_from_slice(&offset.to_le_bytes());
        offset += buf.len() as u32;
    }

    for (_, buf) in png_layers {
        ico.extend_from_slice(buf);
    }

    ico
}

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
