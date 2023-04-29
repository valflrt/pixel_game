#![forbid(unsafe_code)]

use color::Color;
use mat::Mat;
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

mod color;
mod mat;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("game")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let grid = Grid::new(WIDTH, HEIGHT);

    event_loop.run(move |event, _, _| {
        if let Event::RedrawRequested(_) = event {
            grid.draw(pixels.frame_mut());
            pixels.render().unwrap()
        }

        if input.update(&event) {}
    });
}

struct Grid {
    mat: Mat<Color>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Grid {
            mat: Mat::new(Color::new(0, 0, 0, 255), (width as usize, height as usize)),
        }
    }

    pub fn draw(&self, pixels: &mut [u8]) {
        for (c, pix) in self.mat.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.to_bytes());
        }
    }
}
