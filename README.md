[![crates-badge]][crates]
[![docs-badge]][docs]
![license-badge]

# Airi116-pico-detection-algo-rs

This library is a reimplementation of _Pixel Intensity Comparison-based Object_ (PICO) detection algorithms in Rust:

- `Detector`: Cascade of binary classifiers from [pico];
- `Localizer`: Localization with an ensemble of randomized trees from [picojs](https://github.com/nenadmarkus/picojs) (see `lploc.js`);
- `Shaper`: Alignment with an ensemble of regression trees from [dlib](https://github.com/davisking/dlib) (see `shape_predictor`).

## Example

To run CLI example, which takes an image, finds all faces, detects some landmarks and pupils:

> **NOTE**: [Git LFS](https://git-lfs.github.com/) is needed to resolve binary files with `git clone`.
>
> If you don't want to use Git LFS you can download models (and test image) direct from this repo
> (see **model** column in the table below)
> and put them under [`models/`](./models) directory.

```sh
cargo run --release --example detect-faces -- --models-dir models -i "assets/test.png" --score 35.0 -o result.png
```

Output image `result.png` should be like this:

![visualization example](./assets/result.png)

## Models

Each algorithm requires to be loaded with correspondent binary model.

| model                     | algorithm   | source                             | Description               |
|---------------------------|-------------|------------------------------------|---------------------------|
| [face.detector.bin]       | `Detector`  | [pico]                             | Human face classifier     |
| [pupil.localizer.bin]     | `Localizer` | [puploc]                           | Human eye pupil localizer |
| [face-5.shaper.bin]       | `Shaper`    | [shape_predictor_5_face_landmarks] | Human