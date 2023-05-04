pub mod display;
pub mod grid;
pub mod object;
pub mod physics;

use std::time::{self, Duration};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{color::Color, game::grid::Grid};

// TODO Split Game into two parts: update things and draw
// things.

pub struct Game {
    dims: (u32, u32),
    title: String,

    grid: Grid,

    input: WinitInputHelper,
}

impl Game {
    pub fn run<U, D>(mut self, update: U, draw: D)
    where
        U: Fn(&Game) + 'static,
        D: Fn(&Game, Duration) + 'static,
    {
        let (width, height) = self.dims;

        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            let scaled_size = LogicalSize::new(width as f64 * 3.0, height as f64 * 3.0);
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

        let total_time = time::Instant::now();
        let mut timer = total_time.clone();
        event_loop.run(move |event, _, control_flow| {
            if let winit::event::Event::RedrawRequested(_) = event {
                self.grid.draw(pixels.frame_mut());
                pixels.render().unwrap()
            }

            if self.input.update(&event) {
                if self.input.close_requested() {
                    *control_flow = ControlFlow::Exit
                }

                update(&self);

                window.request_redraw();
            }

            draw(&self, timer.elapsed());

            timer = time::Instant::now();
        });
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }
}

pub struct GameBuilder {
    dims: Option<(u32, u32)>,
    title: Option<String>,
    background_color: Option<Color>,
}

impl GameBuilder {
    pub fn new() -> Self {
        GameBuilder {
            dims: None,
            title: None,
            background_color: None,
        }
    }

    pub fn dims(mut self, dims: (u32, u32)) -> Self {
        self.dims = Some(dims);
        self
    }
    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = Some(background_color);
        self
    }

    pub fn build(self) -> Game {
        let dims = self
            .dims
            .expect("Game should have dims, define them with GameBuilder::dims(dims)..");
        Game {
            dims,
            title: self.title.unwrap_or("Game".to_string()),
            grid: Grid::new(dims, self.background_color),
            input: WinitInputHelper::new(),
        }
    }
}
