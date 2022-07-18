use glium;
use glium::{glutin, glutin::{event_loop, window, dpi}, Surface};
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

glium::implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let events_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(640., 480.)).with_title("Grid Sim");
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

	let image = image::load(Cursor::new(&include_bytes!("image.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	let vertex_shader_src = r#"
		#version 140

		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;
		
		uniform mat4 matrix;
		
		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
	"#;

	let fragment_shader_src = r#"
		#version 140

		in vec2 v_tex_coords;
		out vec4 color;
		
		uniform sampler2D tex;
		
		void main() {
			color = texture(tex, v_tex_coords);
		}
	"#;
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	events_loop.run(move |event, _, control_flow| {
		*control_flow = match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				_ => glutin::event_loop::ControlFlow::Poll
			},
			glutin::event::Event::MainEventsCleared => {
				let mut frame = display.draw();
				frame.clear_color(0.5, 0., 0., 0.);

				let data = &[
					Vertex {
						position: [0.0, 0.0],
						tex_coords: [0.0, 0.0]
					},
					Vertex {
						position: [1.0, 0.0],
						tex_coords: [1.0, 0.0]
					},
					Vertex {
						position: [0.0, 1.0],
						tex_coords: [0.0, 1.0]
					},
				];
				let vertex_buffer = glium::vertex::VertexBuffer::new(&display, data).unwrap();
				let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

				let uniforms = glium::uniform! {
					matrix: [
						[1.0, 0.0, 0.0, 0.0],
						[0.0, 1.0, 0.0, 0.0],
						[0.0, 0.0, 1.0, 0.0],
						[0.0, 0.0, 0.0, 1.0f32],
					],
					tex: &texture,
				};

				frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();

				frame.finish().unwrap();
				glutin::event_loop::ControlFlow::Poll
			}
			_ => {
				//println!("{:?}", event);
				glutin::event_loop::ControlFlow::Poll
			}
		}
	});
}