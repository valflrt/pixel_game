#![forbid(unsafe_code)]

use std::time;

use pixel_game_lib::{
    color::Color,
    drawable::{Animation, Drawable},
    game::{object::Object, GameBuilder},
    mat::Mat,
    physics::Physics,
};
use winit::event::VirtualKeyCode;

const WIDTH: i32 = 128;
const HEIGHT: i32 = 48;

const BG_COLOR: Color = Color {
    r: 240,
    g: 240,
    b: 255,
    a: 255,
};

fn main() {
    let game = GameBuilder::new()
        .dims((2 * WIDTH, 2 * HEIGHT))
        .render_dims((WIDTH, HEIGHT))
        .background_color(BG_COLOR)
        .build();

    let ground = Object::new((10, 44), (WIDTH - 20, (HEIGHT - 44)));
    let mut character = Object::new((16, 0), (8, 18));

    let ground_image = Mat::filled_with(Color::new(40, 40, 50, 255), *ground.dims());
    let mut character_anim: Drawable = Animation::from_files(
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
            &["assets/sprites/walking_1.png"],
        ],
        (24, 24).into(),
    )
    .into();

    let mut flip = (false, false);

    let mut current_frame = character_anim.next().unwrap();

    let mut timer = time::Instant::now();
    let mut anim_timer = time::Instant::now();

    let mut physics = Physics::new(*character.pos(), (0., 0.), 58., 9.81);
    physics.set_tf_to_w();

    let mut n: u8 = 0;
    game.run(move |game| {
        let input = game.input();

        if input.key_held(VirtualKeyCode::Left) {
            flip.0 = true;
            *character_anim.state_mut() = 1;
            physics.v_mut().0 = -20.;
        } else if input.key_held(VirtualKeyCode::Right) {
            flip.0 = false;
            *character_anim.state_mut() = 1;
            physics.v_mut().0 = 20.;
        } else {
            *character_anim.state_mut() = 0;
            physics.v_mut().0 = 0.;
        }

        let grounded = character.colliding_with(&ground);
        if grounded {
            physics.a_mut().1 = 0.;
            physics.v_mut().1 = 0.;
            *physics.tf_mut() = (0., 0.).into();

            if input.key_pressed(VirtualKeyCode::Up) {
                *character_anim.state_mut() = 0;
                physics.v_mut().1 = -50.;
            }
        } else {
            *character_anim.state_mut() = 2;
            physics.set_tf_to_w();
        }

        physics.update(timer.elapsed().as_secs_f64());

        let pos = *physics.pos();
        *character.pos_mut() = pos.into();

        n = (n + 1) % 10;
        if n == 0 {
            println!("v = {:?}", physics.v());
            println!("a = {:?}", physics.a());
            println!("tf = {:?}", physics.tf());
        }

        if anim_timer.elapsed().as_millis() >= 80 {
            current_frame = character_anim.next().unwrap();
            anim_timer = time::Instant::now();
        }

        game.clear(BG_COLOR);
        game.image_at(*ground.pos(), &ground_image.as_slice());
        game.image_at(
            *character.pos(),
            &current_frame.slice((8, 2), (9, 19), flip),
        );

        timer = time::Instant::now();
    });
}
