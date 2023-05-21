#![forbid(unsafe_code)]

use std::time;

use pixel_game_lib::{
    color::Color,
    drawable::{Drawable, UniqueFrame},
    game::{object::Object, GameBuilder},
    physics::Physics,
    shape::Shape,
    vec2::Vec2,
};
use winit::event::VirtualKeyCode;

const WIDTH: i32 = 196;
const HEIGHT: i32 = 128;

const BG_COLOR: Color = Color {
    r: 30,
    g: 30,
    b: 30,
    a: 255,
};

fn main() {
    let game = GameBuilder::new()
        .dims((WIDTH, HEIGHT))
        .background_color(BG_COLOR)
        .build();

    let mut object = Object::new((WIDTH / 2, 10), Shape::Rect(Vec2(2, 2)));

    let mut physics = Physics::new(*object.pos(), (0., 0.), 60., 9.81);

    let mut animation: Drawable = UniqueFrame::from_color(Color::white(), (1, 1)).into();
    let image = animation.next().unwrap();

    physics.set_tf_to_w();

    let total_time = time::Instant::now();
    let mut timer = total_time.clone();

    let mut forces_applied = (false, false, false);

    let w = physics.w();

    game.run(move |game| {
        if game.input().key_held(VirtualKeyCode::Left) {
            if !forces_applied.0 {
                physics.apply_force((-2000., 0.));
                forces_applied.0 = true;
            }
        } else if forces_applied.0 {
            physics.apply_force((2000., 0.));
            forces_applied.0 = false;
        }

        if game.input().key_held(VirtualKeyCode::Right) {
            if !forces_applied.1 {
                physics.apply_force((2000., 0.));
                forces_applied.1 = true;
            }
        } else if forces_applied.1 {
            physics.apply_force((-2000., 0.));
            forces_applied.1 = false;
        }

        if game.input().key_held(VirtualKeyCode::Up) {
            if !forces_applied.2 {
                physics.apply_force((0., -w - 6000.));
                forces_applied.2 = true;
            }
        } else if forces_applied.2 {
            physics.apply_force((0., w + 6000.));
            forces_applied.2 = false;
        }

        let t = timer.elapsed().as_secs_f64();
        physics.update(t);

        let pos = *physics.pos();
        *object.pos_mut() = pos.into();

        println!("t = {}s", total_time.elapsed().as_secs_f32());
        println!("v = {:?}", physics.v());
        println!("a = {:?}", physics.a());
        println!("tf = {:?}", physics.tf());

        game.clear(Color::new(0, 0, 0, 0));
        game.image_at(*object.pos(), &image.as_slice());

        timer = time::Instant::now();
    });
}
