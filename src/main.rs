use glium::glutin::event;
use glium::{self, uniforms, Blend};
use glium::{glutin, glutin::{event_loop, window, dpi}, Surface};
use std::io::Cursor;
mod world;
mod vertex;
mod texture;
mod tile;

fn main() {
	// Create window
	let events_loop = event_loop::EventLoop::new();
	let window_builder = window::WindowBuilder::new()
		.with_inner_size(dpi::LogicalSize::new(640., 480.)).with_title("Grid Sim");
	let context_builder = glutin::ContextBuilder::new();
	let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

	// Create texture
	let image = image::load(Cursor::new(&include_bytes!("textures.png")),
						image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

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

	let mut cursor_world_x = 0;
	let mut cursor_world_y = 0;

	// World
	let mut world = world::World::new();

	// Program loop
	events_loop.run(move |event, _, control_flow| {
		*control_flow = glutin::event_loop::ControlFlow::Poll;
		match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				// On exit button press
				event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
				// On scroll wheel scrool
				event::WindowEvent::MouseWheel { device_id: _, delta, phase: _, ..} => {
					match delta {
						event::MouseScrollDelta::LineDelta(_, y) => {
							zoom_level = (zoom_level - (y as i8)).clamp(-4, 3);
						},
						_ => {}
					}
				}
				// On cursor move
				event::WindowEvent::CursorMoved { device_id: _, position, .. } => {
					let last_cursor_x = cursor_x;
					let last_cursor_y = cursor_y;
					cursor_x = position.x as u16;
					cursor_y = position.y as u16;
					let delta_x = (cursor_x as i16) - (last_cursor_x as i16);
					let delta_y = (cursor_y as i16) - (last_cursor_y as i16);

					let zoom = (2.0f32).powi(-(zoom_level as i32));
					cursor_world_x = ((scroll_x / 16.) + (((cursor_x as i32) - ((window_width as i32) / 2)) as f32) / zoom / 16.) as i32;
					cursor_world_y = ((scroll_y / 16.) + (((cursor_y as i32) - ((window_height as i32) / 2)) as f32) / zoom / 16.) as i32;

					// If right clicking then pan camera
					if is_right_clicking {
						scroll_x = (scroll_x - (delta_x as f32) / zoom).clamp(0., 4096.);
						scroll_y = (scroll_y - (delta_y as f32) / zoom).clamp(0., 4096.);
					}
				}
				// Window resize
				event::WindowEvent::Resized(size) => {
					window_width = size.width as u16;
					window_height = size.height as u16;
				}
				// Mouse click
				event::WindowEvent::MouseInput { device_id: _, state, button, .. } => {
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

					if matches!(button, event::MouseButton::Left) {
						world.build(cursor_world_x, cursor_world_y);
					}
				}
				// Keyboard keypress
				event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
					// Toggle fullscreen if F11 is pressed
					if matches!(input.virtual_keycode.unwrap(), event::VirtualKeyCode::F11) && matches!(input.state, event::ElementState::Released) {
						let is_fullscreen = !matches!(display.gl_window().window().fullscreen(), None);
						display.gl_window().window().set_fullscreen(match is_fullscreen {
							true => None,
							false => Some(window::Fullscreen::Borderless(None))
						});
					}
				}
				_ => {}
			},

			// Draw
			glutin::event::Event::MainEventsCleared => {
				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0.2, 0.8, 1., 0.);

				// Get tris for each tile
				let world_tris = world.render(cursor_world_x, cursor_world_y);

				// Draw tris
				let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &world_tris).unwrap();
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
				// End frame
				frame_counter = frame_counter.wrapping_add(1);
			}

			_ => {}
		}
	});
}