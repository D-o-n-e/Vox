use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use image::{io::Reader as ImageReader, DynamicImage};
use wgpu::*;
use vox_render::*;
use vox_ecs::*;
pub struct Engine;

impl Engine{
    pub fn start(world: World){
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        window.set_title("Vox App");
        for i in 0..world.components.len(){
            world.components[i].0.start(&world.components[i].1, &world);   
        }
        let img = match &world.components[0].1{
            Components::Sprite2D(data) => Some(&data.image),
            _ => {None}
        };
        let mut render = pollster::block_on(Render::new(&window));

        // Main Loop
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !render.input(event){
                // App State
                for c in &world.components{
                    c.0.resources(&world.resources);
                }

                // Events
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        render.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        render.resize(**new_inner_size);
                    }
                    _ => {}
            }},
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                render.update();
                match render.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => render.resize(render.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        });
    }
}

