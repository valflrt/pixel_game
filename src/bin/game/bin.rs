#![forbid(unsafe_code)]

use std::time::{self, Instant};

use pixel_game_lib::{
    color::Color,
    game::GameBuilder,
    mat::Mat,
    object::Object,
    physics::Physics,
    resources::{import_sprite, import_spritesheet},
    shape::Shape,
    uv_map::UvMap,
    vec2::Vec2,
};
use winit::event::VirtualKeyCode;

const DIMS: Vec2 = Vec2(128., 48.);
const BG_COLOR: Color = Color {
    r: 240,
    g: 240,
    b: 255,
    a: 255,
};

const WALKING_SPEED: f64 = 20.;

enum CharacterState {
    Walking,
    Standing,
    Jumping,
}

struct Animation {
    timer: Instant,
    current_frame: usize,
    frames: Vec<Mat<Color>>,
}

fn main() {
    let game = GameBuilder::new()
        .dims(Vec2(2. * DIMS.0, 2. * DIMS.1))
        .render_dims(Vec2(DIMS.0, DIMS.1))
        .background_color(BG_COLOR)
        .build();

    let platform1 = Object::new(Vec2(10., 44.), Shape::new_rect(Vec2(40., 4.)));
    let mut platform2 = platform1.clone();
    platform2.pos_mut().0 = 74.;

    let original_char_pos = Vec2(16., 0.);
    let mut character = Object::new(original_char_pos, Shape::Rect(Vec2(8., 18.)));

    let map = UvMap::new(import_sprite("textures/uv_map.png", (24, 24)));

    let ground_img = Mat::filled_with(Color::new(40, 40, 50, 255), platform1.raw_dims().to_usize());

    let standing_img = import_sprite("sprites/standing.png", (24, 24));

    let walking_frames = import_spritesheet("spritesheets/walking.png", (24, 24), (5, 2), 10);
    let mut walking_anim = Animation {
        timer: Instant::now(),
        current_frame: 0,
        frames: walking_frames,
    };

    let mut flip = false;

    let mut timer = time::Instant::now();

    let mut char_state = CharacterState::Standing;

    let mut physics = Physics::new(*character.pos(), Vec2(0., 0.), 60., 90.);
    physics.set_tf_to_w();

    let mut n: u8 = 0;
    game.run(move |game| {
        let input = game.input();

        if character.pos().1 >= DIMS.1 {
            *character.pos_mut() = original_char_pos;
        }

        let grounded = character.in_contact_with_any(&[&platform1, &platform2]);
        if grounded {
            physics.a_mut().1 = 0.;
            physics.v_mut().1 = 0.;
            *physics.tf_mut() = Vec2(0., 0.);

            if input.key_pressed(VirtualKeyCode::Up) {
                char_state = CharacterState::Standing;
                physics.v_mut().1 = -50.;
            }
        } else {
            char_state = CharacterState::Jumping;
            physics.set_tf_to_w();
        }

        let lateral_moving_speed = if grounded {
            WALKING_SPEED
        } else {
            WALKING_SPEED * 0.8
        };
        if input.key_held(VirtualKeyCode::Left) {
            flip = true;
            char_state = CharacterState::Walking;
            physics.v_mut().0 = -lateral_moving_speed;
        } else if input.key_held(VirtualKeyCode::Right) {
            flip = false;
            char_state = CharacterState::Walking;
            physics.v_mut().0 = lateral_moving_speed;
        } else {
            char_state = CharacterState::Standing;
            physics.v_mut().0 = 0.;
        }

        let elapsed = timer.elapsed().as_secs_f64();
        physics.update(elapsed);

        *character.pos_mut() = *physics.pos();

        n = (n + 1) % 10;
        if n == 0 {
            println!("v = {:?}", physics.v());
            println!("a = {:?}", physics.a());
            println!("tf = {:?}", physics.tf());
        }

        game.clear(BG_COLOR);

        let ground_img_slice = ground_img.as_slice();
        game.image_at(*platform1.pos(), &ground_img_slice);
        game.image_at(*platform2.pos(), &ground_img_slice);
        game.image_at(
            *character.pos(),
            &match char_state {
                CharacterState::Standing => map.render(&standing_img.as_slice()),
                CharacterState::Walking => {
                    if walking_anim.timer.elapsed().as_secs_f64() >= 1. / WALKING_SPEED {
                        walking_anim.current_frame =
                            (walking_anim.current_frame + 1) % walking_anim.frames.len();
                        walking_anim.timer = Instant::now();
                    };
                    map.render(&walking_anim.frames[walking_anim.current_frame].slice(
                        (0, 0),
                        (24, 24),
                        (flip, false),
                    ))
                }
                CharacterState::Jumping => map.render(&standing_img.as_slice()),
            }
            .as_slice(),
        );

        timer = time::Instant::now();
    });
}
