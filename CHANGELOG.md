# Changelog


## &ensp; [` 📦 v0.3.0  `](https://github.com/kh4f/svico/compare/v0.2.0...v0.3.0)

### &emsp; 🎁 Features
- **Clearer conversion logs**: `svico` now shows a `Generating ... from ...` header with the target sizes, right-aligned per-layer sizes, and a `Done!` summary with human-readable `B`/`KiB` counts. [🡥](https://github.com/kh4f/svico/commit/e7baca2)

### &emsp; 📋 Docs
- **Status badges**: the README now shows crates.io version, download counts, and docs.rs badges up top. [🡥](https://github.com/kh4f/svico/commit/a27b5ee)
- **Concise intro**: the tagline is followed by a one-line explanation of how `svico` renders, compresses, and assembles the ICO. [🡥](https://github.com/kh4f/svico/commit/35f6453)

##### &emsp;&emsp; [Commit log](https://github.com/kh4f/svico/compare/v0.2.0...v0.3.0) &ensp;•&ensp; Aug 7, 2026


## &ensp; [` 📦 v0.2.0  `](https://github.com/kh4f/svico/compare/v0.1.0...v0.2.0)

### &emsp; 📢 BREAKING CHANGES
- **Reduced public API**: the library now exposes only the `convert` function; `DEFAULT_SIZES`, `AnyResult`, `render_to_png`, and `build_ico` are no longer exported. [🡥](https://github.com/kh4f/svico/commit/6d98920)

### &emsp; 🎁 Features
- **CLI help output**: added `-h`/`--help` support that prints usage, options, and examples. [🡥](https://github.com/kh4f/svico/commit/a599aa4)
- **Helpful missing-input message**: running without an input SVG now points to `--help`. [🡥](https://github.com/kh4f/svico/commit/1e1962c)

### &emsp; 📋 Docs
- **Project README**: documented CLI usage, options, and examples. [🡥](https://github.com/kh4f/svico/commit/d8c69a4)
- **Library API guide**: README now shows using `svico` as a dependency with a `convert` example. [🡥](https://github.com/kh4f/svico/commit/a9f85cb)
- **Prebuilt binary link**: README points to GitHub Releases for ready-made binaries. [🡥](https://github.com/kh4f/svico/commit/368a65f)

##### &emsp;&emsp; [Commit log](https://github.com/kh4f/svico/compare/v0.1.0...v0.2.0) &ensp;•&ensp; Aug 7, 2026


## &ensp; [` 📦 v0.1.0  `](https://github.com/kh4f/svico/commits/v0.1.0)

### &emsp; 🎁 Features
- **SVG to ICO conversion**: added an initial `.svg` → `.ico` pipeline that renders at 16/24/32/256 px, encodes the layers as PNG, and assembles them into a single ICO file. [🡥](https://github.com/kh4f/svico/commit/b36cb72)
- **Custom output path**: added a positional SVG input and an optional `-o`/`--output` flag with unknown-option detection. [🡥](https://github.com/kh4f/svico/commit/8fca49e)
- **Custom layer sizes**: added the `-s`/`--sizes` option to choose layer sizes (defaults to `16,24,32,256`, validated to the 1–256 range). [🡥](https://github.com/kh4f/svico/commit/75fa69d)

### &emsp; ⚡ Performance
- **Smaller ICO files**: PNG layers are now compressed with oxipng at max compression to shrink the resulting `.ico` size. [🡥](https://github.com/kh4f/svico/commit/9104b4d)

### &emsp; 📋 Docs
- **Documented ICO pipeline**: added documentation for the ICO format and the rendering pipeline. [🡥](https://github.com/kh4f/svico/commit/9532ec7)

##### &emsp;&emsp; [Commit log](https://github.com/kh4f/svico/commits/v0.1.0) &ensp;•&ensp; Aug 6, 2026