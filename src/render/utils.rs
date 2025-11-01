//! Utilities for initializing pixel buffers for rendering.
//!
//! Provides functions to create an initial framebuffer for the screen,
//! either filling it with a default background color or extracting pixel data
//! from a provided background image.

use super::renderer::DEFAULT_BACKGROUND_COLOR;
use crate::screen::{HEIGHT, WIDTH};
use image::{DynamicImage, GenericImageView};

/// Creates a vector filled with the default background color,
/// sized to the screen resolution (`WIDTH` x `HEIGHT`).
///
/// This represents the initial pixel buffer when no background image is available.
///
/// # Returns
/// A vector of RGBA tuples representing screen pixels all set to the default background color.
fn make_init_default_background() -> Vec<(u8, u8, u8, u8)> {
    let mut pixels = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for _ in 0..HEIGHT {
        for _ in 0..WIDTH {
            pixels.push((
                DEFAULT_BACKGROUND_COLOR.0,
                DEFAULT_BACKGROUND_COLOR.1,
                DEFAULT_BACKGROUND_COLOR.2,
                DEFAULT_BACKGROUND_COLOR.3,
            ));
        }
    }
    pixels
}

/// Creates an initial framebuffer from an optional background image.
///
/// If no image is given, or if the image is smaller than screen resolution,
/// this falls back to initializing the buffer with the default background color.
///
/// Otherwise, it copies pixels from the top-left corner of the image to fit the screen.
///
/// # Parameters
/// - `image`: Optional dynamic image providing the background.
///
/// # Returns
/// A vector of RGBA pixel tuples sized for the screen resolution suitable for initial rendering.
// TODO: maybe not top left corner
pub fn make_init_frame(image: Option<DynamicImage>) -> Vec<(u8, u8, u8, u8)> {
    match image {
        None => make_init_default_background(),
        Some(image) => {
            let (width, height) = image.dimensions();
            if width < WIDTH || height < HEIGHT {
                eprintln!(
                    "Error: background image is smaller than screen size; Initialized with default background"
                );
                make_init_default_background()
            } else {
                let mut pixels = Vec::with_capacity((WIDTH * HEIGHT) as usize);
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let pixel = image.get_pixel(x, y);
                        pixels.push((pixel[0], pixel[1], pixel[2], pixel[3]));
                    }
                }
                pixels
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DEFAULT_BACKGROUND: (u8, u8, u8, u8) = (
        DEFAULT_BACKGROUND_COLOR.0,
        DEFAULT_BACKGROUND_COLOR.1,
        DEFAULT_BACKGROUND_COLOR.2,
        DEFAULT_BACKGROUND_COLOR.3,
    );

    #[test]
    fn test_make_init_default_background() {
        let mut vector = make_init_default_background();
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                assert_eq!(vector.pop(), Some(DEFAULT_BACKGROUND));
            }
        }
    }

    #[test]
    fn test_make_init_frame_none() {
        let image = None;
        let mut vector = make_init_frame(image);
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                assert_eq!(vector.pop(), Some(DEFAULT_BACKGROUND));
            }
        }
    }

    #[test]
    fn test_make_init_frame_some_image() {
        let image = DynamicImage::new_rgb8(WIDTH, HEIGHT);
        let mut vector = make_init_frame(Some(image));
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                assert_eq!(vector.pop(), Some((0, 0, 0, 255)));
            }
        }
    }
}
