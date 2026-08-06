# Changelog

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