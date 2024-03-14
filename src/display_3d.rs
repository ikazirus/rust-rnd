// extern crate glium;
// extern crate obj;

// use glium::{glutin, Surface};
// use std::fs::File;
// use std::io::BufReader;

// // Define your vertex and fragment shaders
// const VERTEX_SHADER_SRC: &str = r#"
//     #version 330 core

//     in vec3 position;
//     in vec3 color;
//     out vec3 v_color;

//     uniform mat4 perspective;
//     uniform mat4 model_view;

//     void main() {
//         gl_Position = perspective * model_view * vec4(position, 1.0);
//         v_color = color;
//     }
// "#;

// const FRAGMENT_SHADER_SRC: &str = r#"
//     #version 330 core

//     in vec3 v_color;
//     out vec4 color;

//     void main() {
//         color = vec4(v_color, 1.0);
//     }
// "#;

// // Define your 3D model structure
// struct Model {
//     vertices: Vec<Vertex>,
//     // Add any other necessary model data
// }

// // Define your vertex structure
// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 3],
//     color: [f32; 3],
// }

// // Implement vertex structure conversion for glium
// implement_vertex!(Vertex, position, color);

// impl Model {
//     // Load the model from an OBJ file
//     fn load_from_obj(file_path: &str) -> Model {
//         let file = BufReader::new(File::open(file_path).unwrap());
//         let obj = obj::Obj::load(file).unwrap();

//         let mut vertices = Vec::new();
//         for object in obj.object_iter() {
//             for shape in object.group_iter() {
//                 for idx in shape.indices.iter() {
//                     let vertex = obj::Vertex {
//                         position: obj.position(*idx),
//                         normal: obj.normal(*idx),
//                         texture: obj.texture(*idx),
//                     };
//                     vertices.push(Vertex {
//                         position: vertex.position,
//                         color: [1.0, 1.0, 1.0], // Set color as white for simplicity
//                     });
//                 }
//             }
//         }

//         Model { vertices }
//     }

//     // Render the model
//     fn render(&self, target: &mut glium::Frame, program: &glium::Program) {
//         let vertex_buffer = glium::VertexBuffer::new(&target.display, &self.vertices).unwrap();
//         let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
//         let uniforms = uniform! {};

//         target.draw(
//             &vertex_buffer,
//             &indices,
//             program,
//             &uniforms,
//             &Default::default(),
//         ).unwrap();
//     }
// }

// // Define your application structure
// struct App {
//     display: glium::Display,
//     model: Model,
// }

// impl App {
//     // Initialize your application
//     fn new(display: glium::Display) -> Self {
//         // Load the model from an OBJ file
//         let model = Model::load_from_obj("path_to_your_model.obj");

//         App { display, model }
//     }

//     // Main rendering loop
//     fn run(&mut self) {
//         let mut closed = false;
//         while !closed {
//             let mut target = self.display.draw();
//             target.clear_color(0.0, 0.0, 0.0, 1.0);

//             // Render the model
//             self.model.render(&mut target, &program); // You need to define the program variable

//             target.finish().unwrap();

//             // Check for events (e.g., window close)
//             for event in self.display.poll_events() {
//                 match event {
//                     glutin::Event::WindowEvent { event, .. } => match event {
//                         glutin::WindowEvent::CloseRequested => closed = true,
//                         _ => (),
//                     },
//                     _ => (),
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     // Initialize window and display
//     let mut events_loop = glutin::EventsLoop::new();
//     let wb = glutin::WindowBuilder::new().with_title("Rust 3D Model");
//     let cb = glutin::ContextBuilder::new().with_vsync(true);
//     let display = glium::Display::new(wb, cb, &events_loop).unwrap();

//     // Create and run the application
//     let mut app = App::new(display);
//     app.run();
// }
