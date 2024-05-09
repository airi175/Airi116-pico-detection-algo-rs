[![crates-badge]][crates]
[![docs-badge]][docs]
![license-badge]

# Airi116-pico-detection-algo-rs

This library is a reimplementation of _Pixel Intensity Comparison-based Object_ (PICO) detection algorithms in Rust:

- `Detector`: Cascade of binary classifiers from [pico];
- `Localizer`: Localization with an ensemble of randomized trees from [picojs](https://github.com/nenadmarkus/picojs) (see `lploc.js`);
- `Shaper`: Alignment with an ensemble of regression trees from [dlib](https://github.com/davisking/dlib) (see `shape_predictor`).

## Example

To run CLI example, which takes an imag