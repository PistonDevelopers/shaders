#![feature(io, path)]
#![feature(plugin)]
#![plugin(gfx_macros)]

extern crate geometry;
extern crate start_piston;
#[macro_use]
extern crate piston;
extern crate shader_version;
extern crate wavefront_obj;
extern crate cam;
extern crate gfx;

use std::default::Default;
use piston::quack::{ Get, Action };

fn main() {
    start_piston::start(
        shader_version::OpenGL::_3_2,
        piston::window::WindowSettings {
            title: "Piston-Shaders Demo: Viewer".to_string(),
            size: [640, 480],
            samples: 0,
            fullscreen: false,
            exit_on_esc: true
        },
        || start()
    );
}

#[vertex_format]
pub struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            pos: [0.0; 3],
            uv: [0.0; 2],
            normal: [0.0; 3],
        }
    }
}

quack! {
    obj: Vertex[]
    get:
    set:
        fn (val: geometry::Position) [] { obj.pos = val.0 }
        fn (val: geometry::TextureCoords) [] { obj.uv = val.0 }
        fn (val: geometry::Normal) [] { obj.normal = val.0 }
    action:
}

pub struct App {
    geometries: geometry::Geometry,
    objects: geometry::Object,
    models: geometry::Model,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    projection: cam::CameraPerspective,
}

impl App {
    fn new(win_size: [u32; 2]) -> App {
        App {
            geometries: geometry::Geometry::new(),
            objects: geometry::Object::new(),
            models: geometry::Model::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            projection: cam::CameraPerspective {
                fov: 90.0f32,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: (win_size[0] as f32) / (win_size[1] as f32)
            }
        }
    }

    fn load_asset(&mut self, asset: &str) {
        use std::old_io::File;

        let txt = File::open(&Path::new(asset)).unwrap().read_to_string().unwrap();
        let obj_set = wavefront_obj::obj::parse(txt).unwrap();
        self.models.action(geometry::Model::add_model(
            &obj_set,
            &mut self.vertices,
            &mut self.indices,
            &mut self.geometries,
            &mut self.objects,
        ));
    }
}


fn start() {
    use piston::window::Size;

    let Size(size) = start_piston::current_window().get();
    let mut app = App::new(size);
    app.load_asset("./assets/red-cube.obj");

    for e in start_piston::events() {

    }
}
