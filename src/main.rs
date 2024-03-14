#[macro_use]
extern crate glium;
extern crate glutin;

use glium::{Display, Surface};
use glium::glutin::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, ContextBuilder};
use std::time::{Duration, Instant};
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::{VertexBuffer};

// Define vertex structure
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

// Vertices of a cube
const VERTICES: [Vertex; 8] = [
    Vertex { position: [-0.5, -0.5, -0.5] },
    Vertex { position: [ 0.5, -0.5, -0.5] },
    Vertex { position: [ 0.5,  0.5, -0.5] },
    Vertex { position: [-0.5,  0.5, -0.5] },
    Vertex { position: [-0.5, -0.5,  0.5] },
    Vertex { position: [ 0.5, -0.5,  0.5] },
    Vertex { position: [ 0.5,  0.5,  0.5] },
    Vertex { position: [-0.5,  0.5,  0.5] },
];

// Indices of the cube's faces
const INDICES: [u16; 36] = [
    0, 1, 2, 2, 3, 0,
    1, 5, 6, 6, 2, 1,
    5, 4, 7, 7, 6, 5,
    4, 0, 3, 3, 7, 4,
    3, 2, 6, 6, 7, 3,
    0, 4, 5, 5, 1, 0,
];

// Rotation speed in radians per second
const ROTATION_SPEED: f32 = 0.5;

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Rotating 3D Cube");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = VertexBuffer::new(&display, &VERTICES).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &INDICES).unwrap();

    let start_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let elapsed = start_time.elapsed();
        let rotation_angle = (elapsed.as_secs_f32() * ROTATION_SPEED).cos() * std::f32::consts::PI;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::RedrawRequested(_) => {
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                let model_matrix = Matrix4::from_axis_angle(&Vector3::z_axis(), rotation_angle);
                let view_matrix = Matrix4::look_at_rh(Point3::new(0.0, 0.0, 2.0), Point3::origin(), Vector3::z());
                let projection_matrix = Matrix4::new_perspective(1.0, display.get_framebuffer_dimensions().unwrap().into(), 0.1, 100.0);

                let uniforms = uniform! {
                    model: model_matrix.into(),
                    view: view_matrix.into(),
                    projection: projection_matrix.into(),
                };

                target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
                target.finish().unwrap();
            },
            _ => (),
        }
    });
}
