#![forbid(unsafe_code)]

use color::Color;
use mat::Mat;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
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

    let mut grid = Grid::new(WIDTH, HEIGHT);

    let mut position: (usize, usize) = (0, 0);
    let mut prev_position: (usize, usize) = (0, 0);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            grid.draw(pixels.frame_mut());
            pixels.render().unwrap()
        }

        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit
            }

            let x = if input.key_held(VirtualKeyCode::Left) {
                Move::Backward
            } else if input.key_held(VirtualKeyCode::Right) {
                Move::Forward
            } else {
                Move::None
            };

            let y = if input.key_held(VirtualKeyCode::Up) {
                Move::Backward
            } else if input.key_held(VirtualKeyCode::Down) {
                Move::Forward
            } else {
                Move::None
            };

            match x {
                Move::Forward => {
                    if position.0 != grid.dims().0 - 1 {
                        position.0 += 1;
                    } else {
                        position.0 = 0;
                    }
                }
                Move::Backward => {
                    if position.0 != 0 {
                        position.0 -= 1;
                    } else {
                        position.0 = grid.dims().0 - 1;
                    }
                }
                Move::None => {}
            };

            match y {
                Move::Forward => {
                    if position.1 != grid.dims().1 - 1 {
                        position.1 += 1;
                    } else {
                        position.1 = 0;
                    }
                }
                Move::Backward => {
                    if position.1 != 0 {
                        position.1 -= 1;
                    } else {
                        position.1 = grid.dims().1 - 1;
                    }
                }
                Move::None => {}
            };

            grid.update(position, prev_position);
            prev_position = position;
            window.request_redraw()
        }
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

    pub fn update(&mut self, position: (usize, usize), prev_position: (usize, usize)) {
        self.mat[prev_position] = Color::new(0, 0, 0, 255);
        self.mat[position] = Color::new(255, 255, 255, 255);
    }

    pub fn draw(&self, pixels: &mut [u8]) {
        for (c, pix) in self.mat.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.to_bytes());
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        self.mat.dims()
    }
}

enum Move {
    Forward,
    Backward,
    None,
}
