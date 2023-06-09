mod grid;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{color::Color, game::grid::Grid, mat::MatSlice, vec2::Vec2};

pub struct Game {
    dims: Vec2,
    render_pos: Vec2,
    render_dims: Vec2,
    title: String,

    grid: Grid,

    input: WinitInputHelper,
}

impl Game {
    pub fn run<U>(mut self, mut update: U)
    where
        U: FnMut(&mut Game) + 'static,
    {
        let Vec2(width, height) = self.render_dims;

        let event_loop = EventLoop::new();

        println!("{:?}", self.render_dims);

        let window = {
            let size = LogicalSize::new(width, height);
            let scaled_size = LogicalSize::new(width * 5.0, height * 5.0);
            WindowBuilder::new()
                .with_title(self.title.clone())
                .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(width as u32, height as u32, surface_texture).unwrap()
        };

        event_loop.run(move |event, _, control_flow| {
            if let winit::event::Event::RedrawRequested(_) = event {
                self.draw_to_window(pixels.frame_mut());
                pixels.render().unwrap()
            }

            if self.input.update(&event) {
                if self.input.close_requested() {
                    *control_flow = ControlFlow::Exit
                }

                update(&mut self);

                window.request_redraw();
            }
        });
    }

    pub fn image_at(&mut self, pos: Vec2, image: &impl MatSlice<Color>) -> Vec<Vec2> {
        let image_dims = image.slice_dims();

        let mut changed_pixels = Vec::new();
        for x in 0..image_dims.0 {
            for y in 0..image_dims.1 {
                let index: Vec2 = pos + Vec2::from_usize(x, y);
                let max_render_pos = self.render_pos + self.render_dims;

                if 0. <= index.0
                    && 0. <= index.1
                    && index.0 <= max_render_pos.0
                    && index.1 <= max_render_pos.1
                {
                    let pixel = image[(x, y)];
                    if pixel.a == 255 && self.grid.mat().has(index.to_usize()) {
                        self.grid.mat_mut()[index.to_usize()] = pixel;
                        changed_pixels.push(index);
                    }
                }
            }
        }
        changed_pixels
    }

    pub fn clear(&mut self, color: Color) {
        let dims = self.grid.dims().to_usize();
        for x in 0..dims.0 {
            for y in 0..dims.1 {
                self.grid.mat_mut()[(x, y)] = color;
            }
        }
    }

    pub fn dims(&self) -> &Vec2 {
        &self.dims
    }

    pub fn pos(&self) -> &Vec2 {
        &self.render_pos
    }
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.render_pos
    }

    pub fn input(&self) -> &WinitInputHelper {
        &self.input
    }

    fn draw_to_window(&self, pixels: &mut [u8]) {
        for (c, pix) in self
            .grid
            .mat()
            .slice(
                self.render_pos.to_usize(),
                self.render_dims.to_usize(),
                (false, false),
            )
            .to_vec()
            .iter()
            .zip(pixels.chunks_exact_mut(4))
        {
            pix.copy_from_slice(&c.to_bytes());
        }
    }
}

pub struct GameBuilder {
    dims: Option<Vec2>,
    render_dims: Option<Vec2>,
    render_pos: Option<Vec2>,
    title: Option<String>,
    background_color: Option<Color>,
}

impl GameBuilder {
    pub fn new() -> Self {
        GameBuilder {
            dims: None,
            render_dims: None,
            render_pos: None,
            title: None,
            background_color: None,
        }
    }

    pub fn dims(mut self, dims: Vec2) -> Self {
        self.dims = Some(dims);
        self
    }
    pub fn render_pos(mut self, render_pos: Vec2) -> Self {
        self.render_pos = Some(render_pos);
        self
    }
    pub fn render_dims(mut self, render_dims: Vec2) -> Self {
        self.render_dims = Some(render_dims);
        self
    }
    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = Some(background_color);
        self
    }

    pub fn build(self) -> Game {
        let dims = self
            .dims
            .expect("Game should have dims, define them with GameBuilder::dims(dims).");

        Game {
            dims,

            render_dims: self.render_dims.unwrap_or(dims),
            render_pos: self.render_pos.unwrap_or(Vec2::ZERO),

            title: self.title.unwrap_or("Game".to_string()),

            grid: Grid::new(dims.into(), self.background_color),
            input: WinitInputHelper::new(),
        }
    }
}
