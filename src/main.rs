use glium::{self, uniforms, Blend};
use glium::{glutin, glutin::{event_loop, window, dpi}, texture, Surface};
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texture_position: [f32; 2],
}

glium::implement_vertex!(Vertex, position, texture_position);

fn main() {
	// Create window
    let events_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(640., 480.)).with_title("Grid Sim");
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

	// Create texture
	let image = image::load(Cursor::new(&include_bytes!("image.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = texture::SrgbTexture2d::new(&display, image).unwrap();

	// Create program
	let vertex_shader = include_str!("vertex_shader.glsl");
	let fragment_shader = include_str!("fragment_shader.glsl");
	let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

	// Behavior
	let behavior = uniforms::SamplerBehavior {
		minify_filter: uniforms::MinifySamplerFilter::Nearest,
		magnify_filter: uniforms::MagnifySamplerFilter::Nearest,
		..Default::default()
	};
	let draw_parameters = glium::DrawParameters {
		blend: Blend::alpha_blending(),
		..glium::DrawParameters::default()
	};

	// Program loop
	events_loop.run(move |event, _, control_flow| {
		*control_flow = match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				_ => glutin::event_loop::ControlFlow::Poll
			},

			// Draw
			glutin::event::Event::MainEventsCleared => {
				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0.2, 0.8, 1., 0.);

				let data = &[
					Vertex {
						position: [0., 0.],
						texture_position: [0., 0.]
					},
					Vertex {
						position: [1., 0.],
						texture_position: [1., 0.]
					},
					Vertex {
						position: [0., 1.],
						texture_position: [0., 1.]
					},
					Vertex {
						position: [0.1, 0.1],
						texture_position: [0., 0.]
					},
					Vertex {
						position: [1.1, 0.1],
						texture_position: [1., 0.]
					},
					Vertex {
						position: [0.1, 1.1],
						texture_position: [0., 1.]
					},
				];

				// Draw
				let vertex_buffer = glium::vertex::VertexBuffer::new(&display, data).unwrap();
				let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
				let uniforms = glium::uniform! {
					matrix: [
						[1., 0., 0., 0.],
						[0., 1., 0., 0.],
						[0., 0., 1., 0.],
						[0., 0., 0., 1.0f32],
					],
					texture_sampler: uniforms::Sampler(&texture, behavior),
				};
				frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();
				frame.finish().unwrap();
				glutin::event_loop::ControlFlow::Poll
			}

			_ => {
				glutin::event_loop::ControlFlow::Poll
			}
		}
	});
}