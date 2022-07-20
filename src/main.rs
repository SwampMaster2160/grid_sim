use glium::glutin::event;
use glium::{self, uniforms, Blend};
use glium::{glutin, glutin::{event_loop, window, dpi}, texture, Surface};
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texture_position: [f32; 2],
}

fn generate_tile(x: u16, y: u16) -> [Vertex; 6] {
	let x_start = (x * 16) as f32;
	let x_end = ((x + 1) * 16) as f32;
	let y_start = (y * 16) as f32;
	let y_end = ((y + 1) * 16) as f32;
	[Vertex {
		position: [x_start, y_start],
		texture_position: [0., 1.]
	},
	Vertex {
		position: [x_end, y_start],
		texture_position: [1., 1.]
	},
	Vertex {
		position: [x_start, y_end],
		texture_position: [0., 0.]
	},
	Vertex {
		position: [x_end, y_start],
		texture_position: [1., 1.]
	},
	Vertex {
		position: [x_end, y_end],
		texture_position: [1., 0.]
	},
	Vertex {
		position: [x_start, y_end],
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
	let mut window_width = 0u16;
	let mut window_height = 0u16;
	let mut is_left_clicking = false;
	let mut is_middle_clicking = false;
	let mut is_right_clicking = false;

	// Program loop
	events_loop.run(move |event, _, control_flow| {
		*control_flow = match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				event::WindowEvent::MouseWheel { device_id: _, delta, phase: _, ..} => {
					//println!("{:?}", delta);
					match delta {
						event::MouseScrollDelta::LineDelta(_, y) => {
							zoom_level = (zoom_level - (y as i8)).clamp(-4, 4);
							//println!("{}", (2.0f32).powi(-(zoom_level as i32)))
							//println!("{}", zoom_level);
						},
						_ => {}
					}
					glutin::event_loop::ControlFlow::Poll
				}
				event::WindowEvent::CursorMoved { device_id: _, position, .. } => {
					let last_cursor_x = cursor_x;
					let last_cursor_y = cursor_y;
					cursor_x = position.x as u16;
					cursor_y = position.y as u16;
					let delta_x = (cursor_x as i16) - (last_cursor_x as i16);
					let delta_y = (cursor_y as i16) - (last_cursor_y as i16);

					//println!("{}", is_right_clicking);
					if is_right_clicking {
						let zoom = (2.0f32).powi(-(zoom_level as i32));
						scroll_x = (scroll_x - (delta_x as f32) / zoom).clamp(0., 4096.);
						scroll_y = (scroll_y - (delta_y as f32) / zoom).clamp(0., 4096.);
						//println!("{}, {}", scroll_x, scroll_y);
					}
					//println!("{}, {}", delta_x, delta_y);
					glutin::event_loop::ControlFlow::Poll
				}
				event::WindowEvent::Resized(size) => {
					window_width = size.width as u16;
					window_height = size.height as u16;
					//println!("{}, {}", window_width, window_height);
					glutin::event_loop::ControlFlow::Poll
				}
				event::WindowEvent::MouseInput { device_id: _, state, button, .. } => {
					//println!("{:?}, {:?}", button, state);
					let button_state: &mut bool = match button {
						event::MouseButton::Left => &mut is_left_clicking,
						event::MouseButton::Middle => &mut is_middle_clicking,
						event::MouseButton::Right => &mut is_right_clicking,
						_ => &mut is_left_clicking
					};
					match state {
						event::ElementState::Pressed => *button_state = true,
						event::ElementState::Released => *button_state = false
					}
					//println!("{}", *button_state);
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

				let mut data: Vec<Vertex> = Vec::new();
				for x in 0..256u16 {
					for y in 0..256u16 {
						data.extend(generate_tile(x, y));
					}
				}

				// Draw
				let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &data).unwrap();
				let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
				let zoom = (2.0f32).powi(-(zoom_level as i32)) / ((window_height as f32) / 2.);
				let aspect_ratio = (window_width as f32) / (window_height as f32);
				let uniforms = glium::uniform! {
					matrix: [
						[zoom / aspect_ratio, 0., 0., 0.],
						[0., -zoom, 0., 0.],
						[0., 0., 0., 0.],
						[(-scroll_x * zoom) / aspect_ratio, scroll_y * zoom, 0., 1.0f32],
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