#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;



gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_position",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        aspect: gfx::Global<f32> = "aspect",
        scale: gfx::Global<f32> = "scale",
        position: gfx::Global<[f32; 2]> = "position",
        out: gfx::RenderTarget<ColorFormat> = "frag_color",
    }
}

const SQUARE: [Vertex; 4] = [
    Vertex { pos: [  1.0,  1.0 ]}, // top right
    Vertex { pos: [ -1.0,  1.0 ]}, // top left
    Vertex { pos: [ -1.0, -1.0 ]}, // bottom left
    Vertex { pos: [  1.0, -1.0 ]}, // bottom right
];

const SQUARE_INDICES: [u16; 6] = [
    0, 1, 2,
    0, 2, 3,
];

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("gfx+Mandelbrot test".to_string())
        .with_dimensions(1280, 720)
        .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/square.vs"),
        include_bytes!("shaders/square.fs"),
        pipe::new()
    );

    let pso = match pso {
        Ok(s) => s,
        Err(e) => {println!("error creating pipeline state object: \n{}", e); panic!("error creating pipeline")},
    };
    
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, &SQUARE_INDICES[..]);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        aspect: {let (x,y) = window.get_inner_size_pixels().unwrap(); (x as f32)/(y as f32)},
        scale: 2.5,
        position: [-0.5, 0.0],
        out: main_color
    };

    let mut running = true;

    while running {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => {running = false},
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::PageUp)) => {data.scale -= 0.2*data.scale;},
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::PageDown)) => {data.scale += 0.2*data.scale;},
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Left)) => {data.position[0] -= 0.2*data.scale;}
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Right)) => {data.position[0] += 0.2*data.scale;}
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Up)) => {data.position[1] += 0.2*data.scale;}
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Down)) => {data.position[1] -= 0.2*data.scale;}
                glutin::Event::Resized(_width, _height) => {
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    data.aspect = {let (x,y) = window.get_inner_size_pixels().unwrap(); (x as f32)/(y as f32)};
                },
                _ => {},
            }
        }
        // draw a frame
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

}
