//! src/main.rs

// TODO: Replace sdl2 with winit and
// try to achieve the same result!
extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("OpenGL :: Shenanigans", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // TODO: render window contents
    }
}
