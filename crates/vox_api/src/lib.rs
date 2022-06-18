pub use winit::{
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
    pub fn start(mut world: World){
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        window.set_title("Vox App");
        let mut render = pollster::block_on(Render::new(&window));
        for i in &mut world.components{
            render.add_instance(i.1); 
            i.0.start(&mut i.1);  
        }
        // let img = match &world.components[0].1{
        //     Components::Sprite2D(data) => Some(&data.image),
        //     _ => {None}
        // };    
        // Main Loop
        event_loop.run(move |event, _, control_flow| 
        // App State
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !render.input(event){
                // Events
                match event {
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } =>{
                       if let Some(ch) = input.virtual_keycode{
                         match ch {
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            _ => {}
                         }
                         for c in &mut world.components{
                            c.0.input(&mut c.1, ch ); 
                        }
                       }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
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
                // App State
                for c in &mut world.components{
                    c.0.update(&mut c.1);
                    //c.0.input(current_input);
                    c.0.resources(&world.resources);                  
                    render.update_instance(c.1);  
                }
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

