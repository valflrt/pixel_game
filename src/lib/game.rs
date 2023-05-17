pub mod grid;
pub mod object;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{color::Color, game::grid::Grid, mat::MatSlice, vec::Vec2};

// TODO Split Game into two parts: update things and draw
// things.

pub struct Game {
    dims: Vec2<i32>,
    render_pos: Vec2<i32>,
    render_dims: Vec2<i32>,
    title: String,

    grid: Grid,

    input: WinitInputHelper,
}

impl Game {
    pub fn run<U>(mut self, mut update: U)
    where
        U: FnMut(&mut Game) + 'static,
    {
        let (width, height) = self.render_dims.into();

        let event_loop = EventLoop::new();

        println!("{:?}", self.render_dims);

        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            let scaled_size = LogicalSize::new(width as f64 * 5.0, height as f64 * 5.0);
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

    pub fn image_at(&mut self, pos: Vec2<i32>, image: &MatSlice<Color>) -> Vec<Vec2<i32>> {
        let image_dims = image.dims();

        let mut changed_pixels = Vec::new();
        for x in 0..image_dims.0 {
            for y in 0..image_dims.1 {
                let index: Vec2<i32> = (pos.0 + x as i32, pos.1 + y as i32).into();
                let max_render_pos = (
                    self.render_pos.0 + self.render_dims.0 - 1,
                    self.render_pos.1 + self.render_dims.1 - 1,
                );

                if 0 <= index.0
                    && 0 <= index.1
                    && index.0 <= max_render_pos.0
                    && index.1 <= max_render_pos.1
                {
                    let pixel = image[(x, y)];
                    if pixel.a == 255 && self.grid.mat().has(index) {
                        self.grid.mat_mut()[index] = pixel;
                        changed_pixels.push(index);
                    }
                }
            }
        }
        changed_pixels
    }

    pub fn clear(&mut self, color: Color) {
        let dims = *self.grid.dims();
        for x in 0..dims.0 {
            for y in 0..dims.1 {
                self.grid.mat_mut()[(x, y)] = color;
            }
        }
    }

    pub fn dims(&self) -> &Vec2<i32> {
        &self.dims
    }

    pub fn pos(&self) -> &Vec2<i32> {
        &self.render_pos
    }
    pub fn pos_mut(&mut self) -> &mut Vec2<i32> {
        &mut self.render_pos
    }

    pub fn input(&self) -> &WinitInputHelper {
        &self.input
    }

    fn draw_to_window(&self, pixels: &mut [u8]) {
        for (c, pix) in self
            .grid
            .mat()
            .slice(self.render_pos, self.render_dims, (false, false))
            .to_mat()
            .iter()
            .zip(pixels.chunks_exact_mut(4))
        {
            pix.copy_from_slice(&c.to_bytes());
        }
    }
}

pub struct GameBuilder {
    dims: Option<Vec2<i32>>,
    render_dims: Option<Vec2<i32>>,
    render_pos: Option<Vec2<i32>>,
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

    pub fn dims<D>(mut self, dims: D) -> Self
    where
        D: Into<Vec2<i32>>,
    {
        self.dims = Some(dims.into());
        self
    }
    pub fn render_pos<P>(mut self, render_pos: P) -> Self
    where
        P: Into<Vec2<i32>>,
    {
        self.render_pos = Some(render_pos.into());
        self
    }
    pub fn render_dims<D>(mut self, render_dims: D) -> Self
    where
        D: Into<Vec2<i32>>,
    {
        self.render_dims = Some(render_dims.into());
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
            render_pos: self.render_pos.unwrap_or((0, 0).into()),

            title: self.title.unwrap_or("Game".to_string()),

            grid: Grid::new(dims.into(), self.background_color),
            input: WinitInputHelper::new(),
        }
    }
}
