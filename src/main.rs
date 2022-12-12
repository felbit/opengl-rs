//! src/main.rs

extern crate sdl2; // TODO: Replace sdl2 with winit and try to achieve the same result!

pub mod render;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("OpenGL :: Shenanigans", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    let vertices: Vec<f32> = vec![
        // position       // color
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // low left
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // low right
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // up middle
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    // upload data to the array buffer
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind buffer after use
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // layout (location = 0)
        gl::VertexAttribPointer(
            0,         // index of the vertex attribute ("layout (location = 0)")
            3,         // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized? (int-to-float conversation)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride
            std::ptr::null(), // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // layout (location = 1)
        gl::VertexAttribPointer(
            1,         // index of the vertex attribute ("layout (location = 1)")
            3,         // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized? (int-to-float conversation)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind array buffer
        gl::BindVertexArray(0); // unbind vertex array
    }

    use std::ffi::CString;
    let vert_shader = render::Shader::from_vert_source(
        &CString::new(include_str!("shaders/triangle.vert")).unwrap(),
    )
    .unwrap();
    let frag_shader = render::Shader::from_frag_source(
        &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader_program.use_program();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
