use glium;
use glium::{glutin, Surface};

fn main() {
	println!("Hello, world!");
	// 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let context_builder = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the window with the events_loop.
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

	loop {
		events_loop.run(move |event, _, control_flow| {
			*control_flow = match event {
				glutin::event::Event::WindowEvent { event, .. } => match event {
					glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
					_ => glutin::event_loop::ControlFlow::Poll
				},
				glutin::event::Event::MainEventsCleared => {
					let mut frame = display.draw();
					frame.clear_color(0.5, 0., 0., 0.);
					frame.finish().unwrap();
					glutin::event_loop::ControlFlow::Poll
				}
				_ => {
					println!("{:?}", event);
					glutin::event_loop::ControlFlow::Poll
				}
			}
		});
	}
}