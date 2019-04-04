#[macro_use]
extern crate glium;
extern crate image;
extern crate glutin;

use std::io::Cursor;

const SHADER_FS: &str = include_str!("../fs.glsl");
const MBROT_FS:  &str = include_str!("../mbrot_fs.glsl");
const SHADER_VS: &str = include_str!("../vs.glsl");

const ZOOM_INC: f32 = 0.05;
const PAN_INC: f64 = 0.05;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).expect("Error creating display");


    let dpi_factor = display.gl_window().window().get_hidpi_factor();

    let image = image::load(Cursor::new(&include_bytes!("../pal.png")[..]),
                            image::PNG).expect("error loading png").to_rgba();
    let image_dimensions = image.dimensions();

    let tex_img: [f32; 16] = [1.0, 1.0, 1.0, 1.0, 
                              0.0, 1.0, 1.0, 1.0,
                              1.0, 0.0, 1.0, 1.0,
                              1.0, 1.0, 0.0, 1.0,];

    let raw = image.into_raw();

    let image = glium::texture::RawImage1d::from_raw_rgba(raw);
    let texture = glium::texture::texture1d::Texture1d::new(&display, image).expect("boi");

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let julia = vec![
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
    ];

//    let julia = vec![
//        Vertex { position: [ 0.0, -1.0], tex_coords: [0.0, 0.0] },
//        Vertex { position: [ 0.0,  1.0], tex_coords: [0.0, 1.0] },
//        Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
//        Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
//    ];

    let julia_vbo = glium::VertexBuffer::new(&display, &julia).expect("Error creating vertex buffer");
    let julia_idx: [u16; 6] = [0, 1, 2, 0, 3, 2];



    let julia_idx = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &julia_idx).expect("Error index buffer");


    let julia_program = glium::Program::from_source(&display, SHADER_VS, SHADER_FS, None).expect("Error creating shader program");


    let mbrot = vec![
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 0.0,  1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 0.0, -1.0], tex_coords: [1.0, 0.0] },
    ];

    let mbrot_vbo = glium::VertexBuffer::new(&display, &mbrot).expect("Error creating vertex buffer");

    let mbrot_idx: [u16; 6] = [0, 1, 2, 0, 3, 2];

    let mbrot_idx = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &mbrot_idx).expect("Error index buffer");

    let mbrot_program = glium::Program::from_source(&display, SHADER_VS, MBROT_FS, None).expect("Error creating shader program");



    let mut closed = false;
    let mut rad: f64 = 2.0;
    let mut center: (f64, f64) = (0.0, 0.0);
    let mut wsize = glutin::dpi::PhysicalSize::new(0.0, 0.0);
    let mut cursor = glutin::dpi::PhysicalPosition { x: 0.0, y: 0.0 };
    let mut c = (0.0, 0.0);

    let mut t = 0.0f64;
    let mut paused = false;

    let mut jside = false;
    

    while !closed {

        let mut target = display.draw();

        let boi: f64 = t.cos();

        let uniforms = uniform! {
            tex: &texture,
            bailout: 100i32,
            c: (t.sin() as f32, 0.5 as f32),
            center: (center.0 as f32, center.1 as f32),
            radius: rad as f32,
        };

        if !paused {t += 0.02;}

        target.draw(&julia_vbo, 
                    &julia_idx, 
                    &julia_program, 
                    &uniforms,
                    &Default::default())
            .expect("Error drawing to buffer");

//        target.draw(&mbrot_vbo, 
//                    &mbrot_idx, 
//                    &mbrot_program, 
//                    &uniforms,
//                    &Default::default())
//            .expect("Error drawing to buffer");

        target.finish().expect("Error finishing draw");

        let (mx, my) = get_true_cursor(cursor, wsize);
        jside = mx > 0.0;

        if !jside {
            c = (2.0 * center.0 + 1.0, center.1);    
        }

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    use glutin::WindowEvent::*;
                    match event {
                        CloseRequested => closed = true,
                        MouseWheel { delta, .. } => {
                            let (wmx, wmy) = get_true_cursor(cursor, wsize);
                            let (cx, cy) = center;
                            let (mx, my) = (wmx * rad + cx, wmy * rad + cy);
                            match delta {
                                glutin::MouseScrollDelta::LineDelta(_, y) => {
                                    rad *= y as f64;
                                },
                                glutin::MouseScrollDelta::PixelDelta(pos) => {
                                    let h = wsize.height;
                                    let h = h as f64;
                                    let y = pos.y as f64;
                                    rad *= (h - y) / h;
                                },

                            }
                            center = (mx - wmx * rad, my - wmy * rad);     
                        },

                        

                        CursorMoved { position, .. } => cursor = position.to_physical(dpi_factor),
                        Resized(size) => wsize = size.to_physical(dpi_factor),

                        MouseInput { .. } => { 
                            
                        },
                   

                        KeyboardInput { input, .. } => { 
                            use glutin::VirtualKeyCode::*;
                            use glutin::ElementState::Pressed;
                            match input.virtual_keycode {
                                Some(keycode) => match keycode {
                                    Up    => center.1 += PAN_INC*rad,
                                    Down  => center.1 -= PAN_INC*rad,
                                    Left  => center.0 -= PAN_INC*rad,
                                    Right => center.0 += PAN_INC*rad,
                                    Space => if let Pressed = input.state {
                                        paused = !paused;
                                    },

                                    _ => ()

                                },

                            None => ()

                        };

                        
                        
                        } 
                        _ => ()
                    }
                },   
             
                _ => (),
            }
        });
    }
}

fn get_true_cursor (cursor: glutin::dpi::PhysicalPosition, size: glutin::dpi::PhysicalSize) -> (f64, f64) {
   (2.0*cursor.x/size.width - 1.0, -(2.0*(cursor.y/size.height) - 1.0))
}
