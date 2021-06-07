# Material Design Color

<div align="center">
    <p>Material Design Color Palettes for Rust</p>
    <p>
        <a href="https://github.com/LeoVen/matdesign-color">
            <img src="https://img.shields.io/badge/Github-matdesign--color-white?logo=github" />
        </a>
        <a href="https://crates.io/crates/matdesign-color">
            <img src="https://img.shields.io/crates/v/matdesign-color.svg" />
        </a>
        <a href="https://docs.rs/matdesign-color/0.1.1/matdesign_color/">
            <img src="https://img.shields.io/badge/docs.rs-matdesign--color-blue" />
        </a>
    </p>
</div>

## Installation

Include this line in your `cargo.toml`

```toml
matdesign-color = "0.1.1"
```

## Examples

You may use the `const fn` through `MatColor` or access them through a specific color.
```rust
use matdesign_color::{MatColor, MatColorRed};

let red1: u32 = MatColor.red().c300();
let brown: u32 = MatColor.brown().c900();
let black: u32 = MatColor.black();

let red2 = MatColorRed.c300();
assert_eq!(red1, red2);
```

Or you may use `MatColor::new` to create colors on the fly:
```rust
use matdesign_color::{MatColor, MatColorVariant, MatColorAccent};

let orange: Option<u32> = MatColor::new(MatColorVariant::Orange, MatColorAccent::A200);
assert!(orange.is_some());

let no_brown: Option<u32> = MatColor::new(MatColorVariant::Brown, MatColorAccent::A200);
assert!(no_brown.is_none());
```

You may also use global constant arrays and index them using an accent.
```rust
use matdesign_color::{MAT_COLORS_RED, MatColor, MatColorAccent};

let red: u32 = MAT_COLORS_RED[MatColorAccent::A700 as usize];
assert_eq!(red, MatColor.red().a700());
```
