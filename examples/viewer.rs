extern crate geometry;
extern crate start_piston;
extern crate piston;
extern crate shader_version;

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

fn start() {
    for e in start_piston::events() {

    }
}
