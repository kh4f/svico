use std::{env, path::PathBuf};

const DEFAULT_SIZES: &[u32] = &[16, 24, 32, 256];

const USAGE: &str = "Usage: svico <input.svg> [options]";

const HELP: &str = "\
Usage: svico <input.svg> [options]

Options:
  -o, --output <path>  Output .ico path [<input>.ico]
  -s, --sizes <list>   Comma-separated sizes in 1..=256 [16,24,32,256]
  -h, --help           Print help

Examples:
  svico icon.svg
  svico icon.svg -o app/favicon.ico
  svico icon.svg -s 64,128,256
";

fn main() -> Result<(), Box<dyn Error>> {
    let mut svg_path = None;
    let mut ico_path = None;
    let mut sizes = DEFAULT_SIZES.to_vec();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{HELP}");
                return Ok(());
            }
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

    svico::convert(&svg_path, &ico_path, &sizes)
}
