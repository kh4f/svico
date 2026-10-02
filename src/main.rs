use std::{
    env,
    path::{Path, PathBuf},
};

const DEFAULT_SIZES: &[u32] = &[16, 24, 32, 256];

const HELP: &str = "\
Usage: svico <input.svg> [options]

Options:
  -o, --output <path>  Output .ico path or directory [<input>.ico]
  -s, --sizes <sizes>  Comma-separated icon sizes in 1..=256 [16,24,32,256]
  -h, --help           Print help

Examples:
  svico icon.svg                          # -> icon.ico (16,24,32,256)
  svico icon.svg -o app/favicon.ico       # -> app/favicon.ico (16,24,32,256)
  svico icon.svg -o icons/ -s 64,128,256  # -> icons/icon.ico (64,128,256)
";

fn main() -> anyhow::Result<()> {
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
                    anyhow::bail!("missing value for -o/--output");
                };
                ico_path = Some(PathBuf::from(value));
            }
            "-s" | "--sizes" => {
                let Some(value) = args.next() else {
                    anyhow::bail!("missing value for -s/--sizes");
                };
                sizes = value
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .parse::<u32>()
                            .map_err(|_| anyhow::anyhow!("invalid size: {s}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
            }
            _ if arg.starts_with('-') => anyhow::bail!("unknown option: {arg}"),
            _ => svg_path = Some(arg),
        }
    }

    if sizes.is_empty() || sizes.iter().any(|&s| s == 0 || s > 256) {
        anyhow::bail!("sizes must be > 0 and < 257");
    }

    let Some(svg_path) = svg_path else {
        eprintln!("{HELP}");
        anyhow::bail!("input SVG path is required");
    };
    let ico_path = resolve_output_path(&svg_path, ico_path)?;

    svico::convert(&svg_path, &ico_path, &sizes)?;
    Ok(())
}

fn resolve_output_path(input_path: &str, output_path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let default_output_path = Path::new(input_path).with_extension("ico");
    let Some(output_path) = output_path else {
        return Ok(default_output_path);
    };

    let output_path_str = output_path.as_os_str().to_string_lossy();
    let is_dir =
        output_path.is_dir() || output_path_str.ends_with('/') || output_path_str.ends_with('\\');

    if !is_dir {
        return Ok(output_path);
    }
    let Some(file_name) = default_output_path.file_name() else {
        anyhow::bail!("input path has no file name");
    };

    Ok(output_path.join(file_name))
}

#[cfg(test)]
mod tests {
    use super::resolve_output_path;
    use std::path::PathBuf;

    #[test]
    fn defaults_to_input_file_name_with_ico_extension() {
        let resolved = resolve_output_path("assets/logo.svg", None).unwrap();

        assert_eq!(resolved, PathBuf::from("assets/logo.ico"));
    }

    #[test]
    fn preserves_explicit_output_file_path() {
        let output = PathBuf::from("icons/custom.ico");

        let resolved = resolve_output_path("assets/logo.svg", Some(output.clone())).unwrap();

        assert_eq!(resolved, output);
    }

    #[test]
    fn appends_input_file_name_when_output_is_existing_directory() {
        let output = std::env::temp_dir();

        let resolved = resolve_output_path("assets/logo.svg", Some(output.clone())).unwrap();

        assert_eq!(resolved, output.join("logo.ico"));
    }

    #[test]
    fn appends_input_file_name_when_output_ends_with_separator() {
        let output = PathBuf::from("icons/");

        let resolved = resolve_output_path("assets/logo.svg", Some(output)).unwrap();

        assert_eq!(resolved, PathBuf::from("icons/logo.ico"));
    }
}
