use anyhow::Result;
use std::fs::read_to_string;
use std::sync::Arc;
use vello::kurbo::{Affine, Line, RoundedRect, Stroke};
use vello::peniko::Color;
use vello::peniko::color::palette;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;

use crate::day9::{Perimeter, create_perimeter, parse_coordinates};
use vello::wgpu;
use winit::platform::windows::EventLoopBuilderExtWindows;

#[derive(Debug)]
enum RenderState {
    /// `RenderSurface` and `Window` for active rendering.
    Active {
        surface: Box<RenderSurface<'static>>,
        valid_surface: bool,
        window: Arc<Window>,
    },
    /// Cache a window so that it can be reused when the app is resumed after being suspended.
    Suspended(Option<Arc<Window>>),
}

struct Day9VisualizerApp {
    context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    state: RenderState,
    scene: Scene,
    perimeter: Perimeter,
}

impl ApplicationHandler for Day9VisualizerApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        // Get the winit window cached in a previous Suspended event or else create a new window
        let window = cached_window
            .take()
            .unwrap_or_else(|| create_winit_window(event_loop));

        // Create a vello Surface
        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Create a vello Renderer for the surface (using its device id)
        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id]
            .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

        // Save the Window and Surface to a state variable
        self.state = RenderState::Active {
            surface: Box::new(surface),
            valid_surface: true,
            window,
        };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.state {
            self.state = RenderState::Suspended(Some(window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Only process events for our window, and only when we have a surface.
        let (surface, valid_surface) = match &mut self.state {
            RenderState::Active {
                surface,
                valid_surface,
                window,
            } if window.id() == window_id => (surface, valid_surface),
            _ => return,
        };

        match event {
            // Exit the event loop when a close is requested (e.g. window's close button is pressed)
            WindowEvent::CloseRequested => event_loop.exit(),

            // Resize the surface when the window is resized
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    self.context
                        .resize_surface(surface, size.width, size.height);
                    *valid_surface = true;
                } else {
                    *valid_surface = false;
                }
            }

            // This is where all the rendering happens
            WindowEvent::RedrawRequested => {
                if !*valid_surface {
                    return;
                }

                // Empty the scene of objects to draw. You could create a new Scene each time, but in this case
                // the same Scene is reused so that the underlying memory allocation can also be reused.
                self.scene.reset();

                // Get the window size
                let width = surface.config.width;
                let height = surface.config.height;

                // Re-add the objects to draw to the scene.
                draw_perimeter(
                    &mut self.scene,
                    &self.perimeter,
                    width as f64,
                    height as f64,
                );

                // Get a handle to the device
                let device_handle = &self.context.devices[surface.dev_id];

                // Render to a texture, which we will later copy into the surface
                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_texture(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &surface.target_view,
                        &vello::RenderParams {
                            base_color: palette::css::BLACK, // Background color
                            width,
                            height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .expect("failed to render to surface");

                // Get the surface's texture
                let surface_texture = surface
                    .surface
                    .get_current_texture()
                    .expect("failed to get surface texture");

                // Perform the copy
                let mut encoder =
                    device_handle
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Surface Blit"),
                        });
                surface.blitter.copy(
                    &device_handle.device,
                    &mut encoder,
                    &surface.target_view,
                    &surface_texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                );
                device_handle.queue.submit([encoder.finish()]);
                // Queue the texture to be presented on the surface
                surface_texture.present();

                device_handle.device.poll(wgpu::PollType::Poll).unwrap();
            }
            _ => {}
        }
    }
}

pub fn render(perimeter: &Perimeter) -> Result<()> {
    let mut app = Day9VisualizerApp {
        context: RenderContext::new(),
        renderers: vec![],
        state: RenderState::Suspended(None),
        scene: Scene::new(),
        perimeter: perimeter.clone(),
    };

    // Create and run a winit event loop
    let event_loop = EventLoop::builder().with_any_thread(true).build()?;
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}

/// Helper function that creates a Winit window and returns it (wrapped in an Arc for sharing between threads)
fn create_winit_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    let attr = Window::default_attributes()
        .with_inner_size(LogicalSize::new(1920, 1080))
        .with_resizable(true)
        .with_title("Vello Shapes");
    Arc::new(event_loop.create_window(attr).unwrap())
}

/// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}

/// Add shapes to a vello scene. This does not actually render the shapes, but adds them
/// to the Scene data structure which represents a set of objects to draw.
fn draw_perimeter(scene: &mut Scene, perimeter: &Perimeter, width: f64, height: f64) {
    let bounds = find_perimeter_bounds(perimeter);

    let perimeter_width = (bounds.x_max - bounds.x_min);
    let perimeter_height = (bounds.y_max - bounds.y_min);

    let width_scale = width / perimeter_width;
    let height_scale = height / perimeter_height;

    for (first, second) in &perimeter.edges[..] {
        let stroke = Stroke::new(1.0);
        let line = Line::new(
            (
                (first.x as f64 - bounds.x_min) * width_scale,
                (first.y as f64 - bounds.y_min) * height_scale,
            ),
            (
                (second.x as f64 - bounds.x_min) * width_scale,
                (second.y as f64 - bounds.y_min) * height_scale,
            ),
        );
        scene.stroke(
            &stroke,
            Affine::IDENTITY,
            Color::new([0., 1., 0., 1.]),
            None,
            &line,
        );

        let stroke = Stroke::new(1.0);
        let rect = RoundedRect::new(
            (first.x as f64 - bounds.x_min) * width_scale,
            (first.y as f64 - bounds.y_min) * height_scale,
            ((first.x as f64 - bounds.x_min) * width_scale) + 1.,
            ((first.y as f64 - bounds.y_min) * height_scale) + 1.,
            1.,
        );
        let rect_stroke_color = Color::new([1., 0., 0., 1.]);
        scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);

        let stroke = Stroke::new(1.0);
        let rect = RoundedRect::new(
            (second.x as f64 - bounds.x_min) * width_scale,
            (second.y as f64 - bounds.y_min) * height_scale,
            ((second.x as f64 - bounds.x_min) * width_scale) + 1.,
            ((second.y as f64 - bounds.y_min) * height_scale) + 1.,
            1.,
        );
        let rect_stroke_color = Color::new([1., 0., 0., 1.]);
        scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);
    }
}

struct Bounds {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

fn find_perimeter_bounds(perimeter: &Perimeter) -> Bounds {
    let mut bounds = Bounds {
        x_min: f64::MAX,
        x_max: f64::MIN,
        y_min: f64::MAX,
        y_max: f64::MIN,
    };

    for (c1, c2) in &perimeter.edges[..] {
        for coordinate in [c1, c2] {
            bounds.x_min = bounds.x_min.min(coordinate.x as f64);
            bounds.x_max = bounds.x_max.max(coordinate.x as f64);
            bounds.y_min = bounds.y_min.min(coordinate.y as f64);
            bounds.y_max = bounds.y_max.max(coordinate.y as f64);
        }
    }

    bounds
}

#[test]
fn test_run_example_part_2() {
    let path = "./puzzle-inputs/day-9-input.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));

    let coordinates = parse_coordinates(example_data.as_str());
    let perimeter = create_perimeter(&coordinates);

    render(&perimeter);
}
