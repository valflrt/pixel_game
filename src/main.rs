use vec2::Vec2D;

mod vec2;

fn main() {
    let mut vec: Vec2D<u32> = Vec2D::new((2, 2));

    vec[(0, 0)] = 5;
}
