use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

const COLOR_A: u32 = 0x00_20_7a_e6;
const COLOR_B: u32 = 0x00_d6_4b_4b;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReproMode {
    Baseline,
    DamageTracked,
}

impl ReproMode {
    pub fn binary_name(self) -> &'static str {
        match self {
            Self::Baseline => "softbuffer-visibility-repro-baseline",
            Self::DamageTracked => "softbuffer-visibility-repro-damage-tracked",
        }
    }

    fn title_prefix(self) -> &'static str {
        match self {
            Self::Baseline => "baseline: always present on redraw",
            Self::DamageTracked => "damage-tracked: skip present when redraw has no logical dirty region",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramePlan {
    PresentFullFrame,
    SkipPresent,
}

#[derive(Debug, Clone, Copy)]
pub struct ReproPlanner {
    mode: ReproMode,
    use_alternate_theme: bool,
    last_presented_color: Option<u32>,
}

impl ReproPlanner {
    pub fn new(mode: ReproMode) -> Self {
        Self {
            mode,
            use_alternate_theme: false,
            last_presented_color: None,
        }
    }

    pub fn toggle_theme(&mut self) {
        self.use_alternate_theme = !self.use_alternate_theme;
    }

    pub fn current_color(self) -> u32 {
        if self.use_alternate_theme {
            COLOR_B
        } else {
            COLOR_A
        }
    }

    pub fn current_color_name(self) -> &'static str {
        if self.use_alternate_theme {
            "RED"
        } else {
            "BLUE"
        }
    }

    pub fn plan_redraw(self) -> FramePlan {
        match self.mode {
            ReproMode::Baseline => FramePlan::PresentFullFrame,
            ReproMode::DamageTracked => {
                if self.last_presented_color == Some(self.current_color()) {
                    FramePlan::SkipPresent
                } else {
                    FramePlan::PresentFullFrame
                }
            }
        }
    }

    pub fn note_presented(&mut self) {
        self.last_presented_color = Some(self.current_color());
    }
}

pub fn run(mode: ReproMode) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let context = Context::new(event_loop.owned_display_handle())?;
    let mut app = ReproApp::new(context, mode);
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct ReproApp {
    context: Context<OwnedDisplayHandle>,
    mode: ReproMode,
    planner: ReproPlanner,
    window: Option<Arc<Window>>,
    surface: Option<Surface<OwnedDisplayHandle, Arc<Window>>>,
}

impl ReproApp {
    fn new(context: Context<OwnedDisplayHandle>, mode: ReproMode) -> Self {
        Self {
            context,
            mode,
            planner: ReproPlanner::new(mode),
            window: None,
            surface: None,
        }
    }

    fn window_title(&self) -> String {
        format!(
            "{} | Space: toggle full-window color | Esc: quit | current={}",
            self.mode.title_prefix(),
            self.planner.current_color_name()
        )
    }

    fn toggle_theme(&mut self) {
        self.planner.toggle_theme();
        if let Some(window) = &self.window {
            window.set_title(&self.window_title());
            window.request_redraw();
        }
        eprintln!(
            "[{}] toggled to {}",
            self.mode.binary_name(),
            self.planner.current_color_name()
        );
    }

    fn render(&mut self) {
        let Some(window) = &self.window else {
            return;
        };
        let Some(surface) = self.surface.as_mut() else {
            return;
        };

        let size = window.inner_size();
        let Some(width) = NonZeroU32::new(size.width.max(1)) else {
            return;
        };
        let Some(height) = NonZeroU32::new(size.height.max(1)) else {
            return;
        };

        surface
            .resize(width, height)
            .expect("failed to resize softbuffer surface");

        match self.planner.plan_redraw() {
            FramePlan::PresentFullFrame => {
                let mut buffer = surface
                    .buffer_mut()
                    .expect("failed to acquire softbuffer buffer");
                buffer.fill(self.planner.current_color());
                buffer.present().expect("failed to present softbuffer frame");
                self.planner.note_presented();
                eprintln!(
                    "[{}] presented full frame in {}",
                    self.mode.binary_name(),
                    self.planner.current_color_name()
                );
            }
            FramePlan::SkipPresent => {
                eprintln!(
                    "[{}] skipped present because redraw had no logical dirty region",
                    self.mode.binary_name()
                );
            }
        }
    }
}

impl ApplicationHandler for ReproApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title(self.window_title())
            .with_inner_size(LogicalSize::new(960.0, 640.0));
        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("failed to create window"),
        );
        let surface =
            Surface::new(&self.context, window.clone()).expect("failed to create softbuffer surface");

        self.surface = Some(surface);
        self.window = Some(window.clone());
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key,
                        ..
                    },
                ..
            } => match logical_key.as_ref() {
                Key::Named(NamedKey::Escape) => event_loop.exit(),
                Key::Named(NamedKey::Space) => self.toggle_theme(),
                _ => {}
            },
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);
    }
}
