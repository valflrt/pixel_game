#![forbid(unsafe_code)]

use std::time;

use pixel_game_lib::{
    color::Color,
    game::{display::Animation, object::ObjectBuilder, physics::Physics, GameBuilder},
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 64;
const HEIGHT: usize = 48;

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
        let scaled_size = LogicalSize::new(WIDTH as f64 * 10.0, HEIGHT as f64 * 10.0);
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

    let mut game = GameBuilder::new()
        .dims((WIDTH, HEIGHT))
        .background_color(BG_COLOR)
        .build();

    let mut character = ObjectBuilder::new()
        .dims((24, 24))
        .display(Animation::from_files(
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
                &["assets/sprites/walking_6.png"],
            ],
            (24, 24),
        ))
        .physics(Physics::new((0., 0.), (0., 0.), 58., 9.81))
        .build();

    character.pos_mut().1 = 27;
    *character.display_mut().state_mut() = 0;

    let mut timer = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            game.grid_mut().draw(pixels.frame_mut());
            pixels.render().unwrap()
        }

        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit
            }

            if input.key_held(VirtualKeyCode::Left) {
                character.display_mut().flip_mut().0 = true;
                *character.display_mut().state_mut() = 1;
                // character.direction.0 = Move::Backward;
            } else if input.key_held(VirtualKeyCode::Right) {
                character.display_mut().flip_mut().0 = false;
                *character.display_mut().state_mut() = 1;
                // character.direction.0 = Move::Forward;
            } else {
                *character.display_mut().state_mut() = 0;
                // character.direction.0 = Move::None;
            };

            if input.key_pressed(VirtualKeyCode::Up) {
                *character.display_mut().state_mut() = 2;
                // character.direction.1 = Move::Backward;
            }

            window.request_redraw();
        }

        if timer.elapsed().as_millis() >= 80 {
            let pos = character.pos_mut();
            // let direction = &mut character.direction;
            // pos.0 = match direction.0 {
            //     Move::Forward => {
            //         if pos.0 + 1 != WIDTH {
            //             pos.0 + 1
            //         } else {
            //             0
            //         }
            //     }
            //     Move::Backward => {
            //         if pos.0 != 0 {
            //             pos.0 - 1
            //         } else {
            //             WIDTH - 1
            //         }
            //     }
            //     Move::None => pos.0,
            // };

            character.display_mut().update();
            character.draw(game.grid_mut());
            timer = time::Instant::now();
        }
    });
}
