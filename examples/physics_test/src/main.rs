#![forbid(unsafe_code)]

use std::time;

use pixel_game::color::Color;
use pixel_game::game::{Grid, Physics, StaticObject};
use pixel_game::vec::Vec2;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 24;
const HEIGHT: usize = 256;

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
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut grid = Grid::new(BG_COLOR, (WIDTH, HEIGHT));

    let mut object = StaticObject::from_color(Color::white(), (1, 1));

    let mut physics = Physics::new(Vec2(0., 0.), Vec2(0., 0.), 20., 8.);

    let mut t_n: u32 = 0;

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
            } else if input.key_held(VirtualKeyCode::Right) {
            } else {
            };

            if input.key_pressed(VirtualKeyCode::Up) {}

            window.request_redraw();
        }

        let t = timer.elapsed().as_secs_f32();
        if t >= 0.01 {
            physics.update(t);
            let pos = physics.pos();
            println!("v: {:?}", physics.v());
            object.object.pos = (pos.0.round() as usize, pos.1.round() as usize);
            timer = time::Instant::now();
            t_n += 1;
            if t_n == 200 {
                physics.add_force(Vec2(0., -320.));
            }
        }

        object.draw(&mut grid);
    });
}
