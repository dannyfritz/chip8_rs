#[macro_use]
extern crate glium;

pub use glium::glutin::Event;
pub use glium::glutin::EventsLoop;
pub use glium::glutin::WindowBuilder;
pub use glium::glutin::WindowEvent;
use glium::index::PrimitiveType;
use glium::texture::unsigned_texture2d::UnsignedTexture2d;
use glium::texture::RawImage2d;
use glium::{glutin, Display, Surface};

pub struct FbNow {
    pub events_loop: EventsLoop,
    display: Display,
    buffer_width: u32,
    buffer_height: u32,
}

impl FbNow {
    pub fn new(window: WindowBuilder, buffer_width: u32, buffer_height: u32) -> FbNow {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        FbNow {
            events_loop,
            display,
            buffer_width,
            buffer_height,
        }
    }
    pub fn update_buffer(&self, buffer: Vec<u8>) {
        let vertex_buffer = {
            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 2],
                tex_coords: [f32; 2],
            }
            implement_vertex!(Vertex, position, tex_coords);
            glium::VertexBuffer::new(
                &self.display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                        tex_coords: [0.0, 1.0],
                    },
                ],
            ).unwrap()
        };
        let index_buffer = glium::IndexBuffer::new(
            &self.display,
            PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        ).unwrap();
        #[allow(redundant_closure)]
        let program = program!(&self.display,
            140 => {
                vertex: "
                    #version 140
                    in vec2 position;
                    in vec2 tex_coords;
                    out vec2 v_tex_coords;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        v_tex_coords = tex_coords;
                    }
                ",
                fragment: "
                    #version 140
                    uniform usampler2D tex;
                    in vec2 v_tex_coords;
                    out vec4 f_color;
                    void main() {
                        f_color = texture(tex, v_tex_coords);
                    }
                "
            },
        ).unwrap();
        let image_data = RawImage2d::from_raw_rgb(buffer, (self.buffer_width, self.buffer_height));
        let tex = UnsignedTexture2d::new(&self.display, image_data).unwrap();
        let uniforms = uniform! {
            tex: &tex,
        };
        let mut target = self.display.draw();
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}
