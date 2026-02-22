use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId};

use crate::gpu::GpuState;
use crate::input::Input;
use crate::{EngineConfig, GameLoop};

struct App<G: GameLoop, F: FnOnce() -> G> {
    config: EngineConfig,
    window: Option<Arc<Window>>,
    gpu: Option<GpuState>,
    game: Option<G>,
    game_factory: Option<F>,
    input: Input,
    last_frame: Option<Instant>,
}

impl<G: GameLoop, F: FnOnce() -> G> App<G, F> {
    fn new(config: EngineConfig, game_factory: F) -> Self {
        Self {
            config,
            window: None,
            gpu: None,
            game: None,
            game_factory: Some(game_factory),
            input: Input::new(),
            last_frame: None,
        }
    }
}

impl<G: GameLoop, F: FnOnce() -> G> ApplicationHandler for App<G, F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let gpu = GpuState::new(window.clone(), self.config.width, self.config.height);
        self.window = Some(window);
        self.gpu = Some(gpu);
        if let Some(factory) = self.game_factory.take() {
            self.game = Some(factory());
        }
        self.last_frame = Some(Instant::now());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(size.width, size.height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => self.input.press(code),
                        ElementState::Released => self.input.release(code),
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta_time = self
                    .last_frame
                    .map(|previous_frame| now.duration_since(previous_frame).as_secs_f32())
                    .unwrap_or(0.0);
                self.last_frame = Some(now);

                if let (Some(game), Some(gpu)) = (&mut self.game, &mut self.gpu) {
                    game.update(&self.input, delta_time, self.config.width, self.config.height);
                    game.draw(gpu);
                    gpu.render();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

pub fn run<G: GameLoop + 'static>(config: EngineConfig, game_factory: impl FnOnce() -> G + 'static) {
    let mut app = App::new(config, game_factory);
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
