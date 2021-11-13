#![no_std]
use palette::{convert::FromColorUnclamped, encoding::srgb::Srgb, rgb::Rgb, Hsv};

/// Provide the width and height of your LED board. If it is a strip, just set
/// either one to 1, whichever makes more sense to you. Using 0 will panic.
///
/// For ms, get an `Instant` at the start of your program and cast the elapsed milliseconds to f32.
pub fn rainbow_emanator(width: u32, height: u32, ms: f32) -> impl Iterator<Item = [u8; 3]> {
    if width == 0 || height == 0 {
        panic!("Do not use 0 for width or height.");
    }
    let x_factor = (width - 1) as f32 / 2.0;
    let y_factor = (height - 1) as f32 / 2.0;
    // Multiplication is faster than division.
    let x_div = 1.0 / x_factor;
    let y_div = 1.0 / y_factor;

    (0..height).flat_map(move |y| {
        (0..width).map(move |x| {
            // Scale extremes to -1..1 and avoid division by 0...
            let x: f32 = if x_factor == 0.0 {
                0.0
            } else {
                (x as f32 - x_factor) * x_div
            };

            let y: f32 = if y_factor == 0.0 {
                0.0
            } else {
                (y as f32 - y_factor) * y_div
            };

            let dist: f32 = palette::float::Float::sqrt(x * x + y * y);
            const TIME_DIV: f32 = 1.0 / 3000.0;
            const VALUE: f32 = 2.0 / 256.0;
            let hsv = Hsv::new((-ms * TIME_DIV + dist * 0.6) * 360.0, 1.0, VALUE);
            let Rgb {
                red, green, blue, ..
            } = Rgb::<Srgb>::from_color_unclamped(hsv).into_format::<u8>();
            [red, green, blue]
        })
    })
}
