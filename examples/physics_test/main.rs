#![forbid(unsafe_code)]

use std::time;

use pixel_game_lib::{
    color::Color,
    game::{
        display::{Displayable, UniqueFrame},
        object::ObjectBuilder,
        physics::Physics,
        GameBuilder,
    },
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 196;
const HEIGHT: u32 = 128;

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

    let mut game = GameBuilder::new()
        .dims((WIDTH, HEIGHT))
        .background_color(BG_COLOR)
        .build();

    let mut object = ObjectBuilder::new()
        .dims((1, 1))
        .display(Displayable::UniqueFrame(UniqueFrame::from_color(
            Color::white(),
            (1, 1),
        )))
        .physics(Physics::new((10., 96.), (0., 0.), 60., 9.81))
        .build();

    object.physics_mut().set_tf_to_w();

    let total_time = time::Instant::now();
    let mut timer = total_time.clone();
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            game.grid_mut().draw(pixels.frame_mut());
            pixels.render().unwrap()
        }

        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                object.physics_mut().set_v((120., -160.));
                println!("hello");
            }

            window.request_redraw();
        }

        // Stops the object to prevent it from going berserk
        // but you can try it by commenting this piece of code.
        if object.pos().1 >= 100 {
            object.physics_mut().reset_all();
        }

        let t = timer.elapsed().as_secs_f32();
        object.physics_mut().update(t);

        let pos = object.physics().s();
        *object.pos_mut() = (pos.0.round() as u32, pos.1.round() as u32);

        println!("t = {}s", total_time.elapsed().as_secs_f32());
        println!("v = {:?}", object.physics().v());
        println!("a = {:?}", object.physics().a());

        timer = time::Instant::now();
        object.draw(game.grid_mut());
    });
}
