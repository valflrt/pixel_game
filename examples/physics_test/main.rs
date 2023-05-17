#![forbid(unsafe_code)]

use std::time;

use pixel_game_lib::{
    color::Color,
    drawable::{Drawable, UniqueFrame},
    game::{object::Object, GameBuilder},
    physics::Physics,
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

    let mut object = Object::new((0, 0), (1, 1));

    let mut physics = Physics::new((10., 96.), (0., 0.), 60., 9.81);

    let mut animation: Drawable = UniqueFrame::from_color(Color::white(), (1, 1)).into();
    let image = animation.next().unwrap();

    physics.set_tf_to_w();

    let total_time = time::Instant::now();
    let mut timer = total_time.clone();

    game.run(move |game| {
        // Stops the object to prevent it from going berserk
        // but you can try it by commenting this piece of code.
        // if object.pos().1 >= 100 && physics.v().1.is_sign_positive() {
        //     physics.reset_all();
        // } else {
        //     physics.set_tf_to_w();
        // }

        if game.input().key_held(VirtualKeyCode::Left) {
            physics.apply_force((-200., 0.));
        } else if game.input().key_held(VirtualKeyCode::Up) {
            physics.apply_force((0., -200.));
        } else if game.input().key_held(VirtualKeyCode::Right) {
            physics.apply_force((200., 0.));
        }

        let t = timer.elapsed().as_secs_f64();
        physics.update(t);

        let pos = *physics.pos();
        *object.pos_mut() = pos.into();

        println!("t = {}s", total_time.elapsed().as_secs_f32());
        println!("v = {:?}", physics.v());
        println!("a = {:?}", physics.a());

        game.clear(Color::new(0, 0, 0, 0));
        game.image_at(*object.pos(), &image.as_slice());

        physics.set_tf_to_w();

        timer = time::Instant::now();
    });
}
