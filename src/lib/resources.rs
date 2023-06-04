use std::{path::PathBuf, str::FromStr};

use crate::{
    color::Color,
    mat::{Mat, MatSlice},
};

const ASSETS_PATH: &str = "assets";

/// Import sprites from spritesheet, `sprite_dims` represents
/// the dimensions of a sprite (that must have the same dims)
/// and `spritesheet_dims` represents the number of rows and
/// columns that the spritesheet holds.
pub fn import_spritesheet(
    path: &str,
    sprite_dims: (usize, usize),
    spritesheet_dims: (usize, usize),
    n_sprites: usize,
) -> Vec<Mat<Color>> {
    let image = image::open(PathBuf::from_str(ASSETS_PATH).unwrap().join(path))
        .unwrap()
        .to_rgba8();
    let image_pixels: Vec<Color> = image
        .as_raw()
        .chunks(4)
        .map(|v| Color {
            r: v[0],
            g: v[1],
            b: v[2],
            a: v[3],
        })
        .collect();
    let image_dims = (
        sprite_dims.0 * spritesheet_dims.0,
        sprite_dims.1 * spritesheet_dims.1,
    );
    let spritesheet_image = Mat::from_vec(image_pixels, image_dims);

    let mut images = Vec::new();

    for x in 0..spritesheet_dims.0 {
        for y in 0..spritesheet_dims.1 {
            if y * spritesheet_dims.0 + x < n_sprites {
                images.push(
                    spritesheet_image
                        .slice(
                            (x * sprite_dims.0, y * sprite_dims.1),
                            sprite_dims,
                            (false, false),
                        )
                        .to_mat(),
                );
            }
        }
    }

    images
}

pub fn import_sprites(paths: &[&str], dims: (usize, usize)) -> Vec<Mat<Color>> {
    let mut images = Vec::new();
    for path in paths {
        let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::open(path).unwrap().to_rgba8();
        images.push(Mat::from_vec(
            image
                .as_raw()
                .chunks(4)
                .map(|v| Color {
                    r: v[0],
                    g: v[1],
                    b: v[2],
                    a: v[3],
                })
                .collect::<Vec<_>>(),
            dims,
        ));
    }
    images
}

pub fn import_sprite(path: &str, dims: (usize, usize)) -> Mat<Color> {
    Mat::from_vec(
        image::open(PathBuf::from_str(ASSETS_PATH).unwrap().join(path))
            .unwrap()
            .to_rgba8()
            .as_raw()
            .chunks(4)
            .map(|v| Color {
                r: v[0],
                g: v[1],
                b: v[2],
                a: v[3],
            })
            .collect::<Vec<_>>(),
        dims,
    )
}
