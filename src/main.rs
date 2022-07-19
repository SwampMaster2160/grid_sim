use glium::glutin::event;
use glium::{self, uniforms, Blend};
use glium::{glutin, glutin::{event_loop, window, dpi}, texture, Surface};
use std::f32::consts::PI;
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texture_position: [f32; 2],
}

fn generate_square(x: f32, y: f32, width: f32) -> [Vertex; 6] {
	[Vertex {
		position: [x, y],
		texture_position: [0., 1.]
	},
	Vertex {
		position: [x + width, y],
		texture_position: [1., 1.]
	},
	Vertex {
		position: [x, y + width],
		texture_position: [0., 0.]
	},
	Vertex {
		position: [x + width, y],
		texture_position: [1., 1.]
	},
	Vertex {
		position: [x + width, y + width],
		texture_position: [1., 0.]
	},
	Vertex {
		position: [x, y + width],
		texture_position: [0., 0.]
	}]
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

	// Vars
	let mut scroll_x = 0.0f32;
	let mut scroll_y = 0.0f32;
	let mut zoom_level = 0i8;
	let mut frame_counter = 0u64;
	let mut cursor_x = 0u16;
	let mut cursor_y = 0u16;

	// Program loop
	events_loop.run(move |event, _, control_flow| {
		*control_flow = match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				event::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
					//println!("{:?}", delta);
					match delta {
						event::MouseScrollDelta::LineDelta(_, y) => {
							zoom_level = (zoom_level + (y as i8)).clamp(-4, 8);
						},
						_ => {}
					}
					glutin::event_loop::ControlFlow::Poll
				}
				event::WindowEvent::CursorMoved { device_id, position, modifiers } => {
					let last_cursor_x = cursor_x;
					let last_cursor_y = cursor_y;
					cursor_x = position.x as u16;
					cursor_y = position.y as u16;
					let delta_x = (cursor_x as i16) - (last_cursor_x as i16);
					let delta_y = (cursor_y as i16) - (last_cursor_y as i16);
					println!("{}, {}", delta_x, delta_y);
					glutin::event_loop::ControlFlow::Poll
				}
				_ => glutin::event_loop::ControlFlow::Poll
			},

			// Draw
			glutin::event::Event::MainEventsCleared => {
				//scroll_x = ((frame_counter as f32) / 100.0f32 * PI).cos();
				//scroll_y = ((frame_counter as f32) / 100.0f32 * PI).sin();

				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0.2, 0.8, 1., 0.);

				let mut data: Vec<Vertex> = Vec::new();// = generate_square(0.2f32, 0.1f32, 0.5f32);
				//data.extend(generate_square(0.2f32, 0.1f32, 0.5f32));
				//data.extend(generate_square(-0.8f32, -0.7f32, 0.2f32));
				for x in 0..64u16 {
					for y in 0..64u16 {
						data.extend(generate_square(x as f32, y as f32, 1.0f32));
					}
				}

				// Draw
				let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &data).unwrap();
				let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
				let zoom = (2.0f32).powi(zoom_level as i32);
				let uniforms = glium::uniform! {
					matrix: [
						[zoom, 0., 0., 0.],
						[0., -zoom, 0., 0.],
						[0., 0., 0., 0.],
						[-scroll_x, scroll_y, 0., 1.0f32],
					],
					texture_sampler: uniforms::Sampler(&texture, behavior),
				};
				frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();
				frame.finish().unwrap();
				frame_counter = frame_counter.wrapping_add(1);
				glutin::event_loop::ControlFlow::Poll
			}

			_ => {
				glutin::event_loop::ControlFlow::Poll
			}
		}
	});
}