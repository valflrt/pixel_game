#![forbid(unsafe_code)]

use std::time;

use color::Color;
use game::{Animation, Grid, Move, Object};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod color;
mod game;
mod mat;

const WIDTH: usize = 64;
const HEIGHT: usize = 24;

const BG_COLOR: Color = Color {
    r: 30,
    g: 30,
    b: 30,
    a: 255,
};

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 5.0, HEIGHT as f64 * 5.0);
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
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut grid = Grid::new(BG_COLOR, (WIDTH, HEIGHT));

    let mut character = Object::new(
        Animation::from_files(
            &[
                &["assets/sprites/standing.png"],
                &[
                    "assets/sprites/walking_1.png",
                    "assets/sprites/walking_2.png",
                    "assets/sprites/walking_3.png",
                    "assets/sprites/walking_4.png",
                    "assets/sprites/walking_5.png",
                    "assets/sprites/walking_6.png",
                    "assets/sprites/walking_7.png",
                    "assets/sprites/walking_8.png",
                    "assets/sprites/walking_9.png",
                    "assets/sprites/walking_10.png",
                ],
            ],
            (24, 24),
        ),
        (24, 24),
    );

    character.pos.1 = 3;
    character.animation.set_state(0);

    let mut timer = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            grid.draw(pixels.frame_mut());
            pixels.render().unwrap()
        }

        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit
            }

            if input.key_held(VirtualKeyCode::Left) {
                character.animation.flip.0 = true;
                character.animation.set_state(1);
                character.direction.0 = Move::Backward;
            } else if input.key_held(VirtualKeyCode::Right) {
                character.animation.flip.0 = false;
                character.animation.set_state(1);
                character.direction.0 = Move::Forward;
            } else {
                character.animation.set_state(0);
                character.direction.0 = Move::None;
            };

            window.request_redraw();
        }

        if timer.elapsed().as_millis() >= 80 {
            character.pos.0 = match character.direction.0 {
                Move::Forward => {
                    if character.pos.0 + 1 != WIDTH {
                        character.pos.0 + 1
                    } else {
                        0
                    }
                }
                Move::Backward => {
                    if character.pos.0 != 0 {
                        character.pos.0 - 1
                    } else {
                        WIDTH
                    }
                }
                Move::None => character.pos.0,
            };
            character.next_frame();
            character.draw(&mut grid);
            timer = time::Instant::now();
        }
    });
}
