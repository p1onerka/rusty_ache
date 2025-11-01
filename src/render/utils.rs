use image::{DynamicImage, GenericImageView};

use crate::screen::{HEIGHT, WIDTH};

use super::renderer::DEFAULT_BACKGROUND_COLOR;

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

/// Get slice of background
/// TODO: maybe not top left corner?
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
