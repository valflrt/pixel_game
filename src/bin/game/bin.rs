#![forbid(unsafe_code)]

use std::time::{self, Instant};

use pixel_game_lib::{
    color::Color,
    game::GameBuilder,
    mat::{Mat, MatSlice},
    object::Object,
    physics::Physics,
    resources::{import_sprite, import_spritesheet},
    uv_map::UvMap,
    vec2::Vec2,
};
use winit::event::VirtualKeyCode;

const DIMS: Vec2 = Vec2(128., 48.);
const BG_COLOR: Color = Color::new(240, 240, 255, 255);

const WALKING_SPEED: f64 = 15.;

enum LateralDirection {
    Left,
    Right,
}

enum CharacterState {
    Walking,
    Standing,
    MidAir,
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

    let platform1 = Object::new(Vec2(10., 44.), Vec2(40., 4.), None);
    let mut platform2 = platform1.clone();
    platform2.pos_mut().0 = 74.;

    let mut character = Object::new(Vec2(24., 0.), Vec2(8., 18.), Some(Vec2(-8., -2.)));

    let map = UvMap::new(import_sprite("textures/uv_map.png", (24, 24)));

    let ground_img = Mat::filled_with(Color::new(40, 40, 50, 255), platform1.dims().to_usize());

    let standing_img = import_sprite("sprites/standing.png", (24, 24));

    let walking_frames = import_spritesheet("spritesheets/walking.png", (24, 24), (5, 2), 10);
    let mut walking_anim = Animation {
        timer: Instant::now(),
        current_frame: 0,
        frames: walking_frames,
    };

    let mut direction = LateralDirection::Right;

    let mut timer = time::Instant::now();

    let mut char_state = CharacterState::Standing;

    let mut physics = Physics::new(*character.pos(), Vec2(0., 0.), 60., 300.);
    physics.set_tf_to_w();

    let start_instant = Instant::now();
    let mut n: u8 = 0;

    let mut prev_grounded = false;
    game.run(move |game| {
        let input = game.input();

        let elapsed = timer.elapsed().as_secs_f64();
        physics.update(elapsed);

        *character.pos_mut() = *physics.pos();

        let grounded = character.in_intersecting_with_any(&[&platform1, &platform2]);

        let lateral_moving_speed = if grounded {
            WALKING_SPEED
        } else {
            WALKING_SPEED * 0.8
        };
        if input.key_held(VirtualKeyCode::Left) {
            direction = LateralDirection::Left;
            char_state = CharacterState::Walking;
            physics.v_mut().0 = -lateral_moving_speed;
        } else if input.key_held(VirtualKeyCode::Right) {
            direction = LateralDirection::Right;
            char_state = CharacterState::Walking;
            physics.v_mut().0 = lateral_moving_speed;
        } else {
            char_state = CharacterState::Standing;
            physics.v_mut().0 = 0.;
        }

        if grounded {
            // character.intersection_boundaries_with(other);
            if !prev_grounded {
                physics.a_mut().1 = 0.;
                physics.v_mut().1 = 0.;
                *physics.tf_mut() = Vec2::ZERO;
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                physics.v_mut().1 = -80.;
            }
        } else {
            char_state = CharacterState::MidAir;
            physics.set_tf_to_w();
        }

        prev_grounded = grounded;

        n = (n + 1) % 6;
        if n == 0 {
            println!("t = {}s", start_instant.elapsed().as_secs_f32());
            println!("v = {:?}", physics.v());
            println!("a = {:?}", physics.a());
            println!("tf = {:?}", physics.tf());
        }

        let flip = match direction {
            LateralDirection::Left => true,
            LateralDirection::Right => false,
        };

        game.clear(BG_COLOR);

        let ground_img_slice = ground_img.as_slice();
        game.image_at(platform1.image_pos(), &ground_img_slice);
        game.image_at(platform2.image_pos(), &ground_img_slice);
        game.image_at(
            character.image_pos(),
            &match char_state {
                CharacterState::Standing => map.render(&standing_img.slice_flip((flip, false))),
                CharacterState::Walking => {
                    if walking_anim.timer.elapsed().as_secs_f64() >= 1.5 / WALKING_SPEED {
                        walking_anim.current_frame =
                            (walking_anim.current_frame + 1) % walking_anim.frames.len();
                        walking_anim.timer = Instant::now();
                    };
                    map.render(
                        &walking_anim.frames[walking_anim.current_frame].slice_flip((flip, false)),
                    )
                }
                CharacterState::MidAir => map.render(&standing_img.slice_flip((flip, false))),
            }
            .as_slice(),
        );

        timer = time::Instant::now();
    });
}
