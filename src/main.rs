mod vertex;
mod matrix44d;
mod vector3d;

#[macro_use]
extern crate glium;

use std::f32::consts::PI;
use std::time::Instant;
use glium::{glutin, Surface};
use crate::glutin::event::{DeviceEvent, WindowEvent};
use crate::matrix44d::Matrix44d;
use crate::vector3d::Vector3d;
use crate::vertex::Vertex;


fn main() {

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    uniform mat4 projection;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 mvp = projection * view * model;
        gl_Position = mvp * vec4(position, 0.0, 1.0);
    }
"#;
    let fragment_shader_src = r#"
    #version 140

    uniform float time;
    uniform vec2 resolution;

    vec3 lightDir = normalize(vec3(1.0, 1.0, 1.0));

    vec3 trans(vec3 pos) {
        return mod(pos, 4.0) - 2.0;
    }

    float dist_func(vec3 pos, float size) {
        return length(trans(pos)) - size;
    }

    vec3 getNormal(vec3 pos, float size) {
        float ep = 0.0001;
        return normalize(vec3(
            dist_func(pos, size) - dist_func(vec3(pos.x - ep, pos.y, pos.z), size),
            dist_func(pos, size) - dist_func(vec3(pos.x, pos.y - ep, pos.z), size),
            dist_func(pos, size) - dist_func(vec3(pos.x, pos.y, pos.z - ep), size)
        ));
    }

    void main( void ) {
        // 解像度からテクスチャとして利用できる`-1～1`の間に正規化する
        vec2 pos = (gl_FragCoord.xy * 2.0 - resolution.xy) / min(resolution.x, resolution.y);

        vec3 col = vec3(0.0);

        vec3 cameraPos = vec3(0.0, 0.0, 1.0);

        vec3 ray = normalize(vec3(pos, 0.0) - cameraPos);
        vec3 cur = cameraPos;

        float size = 0.5;
        for (int i = 0; i < 256; i++) {
            float d = dist_func(cur, size);
            if (d < 0.0001) {
                vec3 normal = getNormal(cur, size);
                float diff = dot(normal, lightDir);
                col = vec3(diff) + vec3(0.1);
                break;
            }
            cur += ray * d;
        }
        gl_FragColor = vec4(col, 1.0);
    }
"#;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.5, -0.5] };
    let vertex3 = Vertex { position: [-0.5, 0.5] };
    let vertex4 = Vertex { position: [0.5, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut camera_position = Vector3d::new(0.0, 0.0, 1.0);

    let mut horizontal_angle: f32 = PI;
    let mut vertical_angle: f32 = 0.0;

    let mut w_pressed = false;
    let mut a_pressed = false;
    let mut s_pressed = false;
    let mut d_pressed = false;
    let mut space_pressed = false;
    let mut shift_pressed = false;


    let projection = Matrix44d::perspective(PI/2.0, 4.0 / 3.0, 0.0001, 1000.0);
    let mut view = Matrix44d::look_at(
        Vector3d::new(0.0, 0.0, 1.0),
        Vector3d::new(0.0, 0.0, -1.0),
        Vector3d::new(0.0, 1.0, 0.0)
    );
    let model = Matrix44d::translate(0.0, 0.0, -1.0);

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut time = std::time::Instant::now();
    let mut ticks: f32 = 0.0;

    event_loop.run(move |ev, _, control_flow| {
        let start_time = std::time::Instant::now();
        let delta_time = start_time.duration_since(time);

        match ev {
            glutin::event::Event::DeviceEvent {event, ..} => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    horizontal_angle -= delta.0 as f32 / 500.0;
                    vertical_angle -= delta.1 as f32 / 500.0;
                    return;
                },
                _ => {return;}
            }
            glutin::event::Event::WindowEvent {event , ..} => match event {
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    match input.state {
                        glutin::event::ElementState::Pressed => {
                            match input.virtual_keycode {
                                Some(glutin::event::VirtualKeyCode::W) => {
                                    w_pressed = true;
                                },
                                Some(glutin::event::VirtualKeyCode::A) => {
                                    a_pressed = true;
                                },
                                Some(glutin::event::VirtualKeyCode::S) => {
                                    s_pressed = true;
                                },
                                Some(glutin::event::VirtualKeyCode::D) => {
                                    d_pressed = true;
                                },
                                Some(glutin::event::VirtualKeyCode::Space) => {
                                    space_pressed = true;
                                },
                                Some(glutin::event::VirtualKeyCode::LShift) => {
                                    shift_pressed = true;
                                },
                                _ => (),
                            }
                        },
                        glutin::event::ElementState::Released => {
                            match input.virtual_keycode {
                                Some(glutin::event::VirtualKeyCode::W) => {
                                    w_pressed = false;
                                },
                                Some(glutin::event::VirtualKeyCode::A) => {
                                    a_pressed = false;
                                },
                                Some(glutin::event::VirtualKeyCode::S) => {
                                    s_pressed = false;
                                },
                                Some(glutin::event::VirtualKeyCode::D) => {
                                    d_pressed = false;
                                },
                                Some(glutin::event::VirtualKeyCode::Space) => {
                                    space_pressed = false;
                                },
                                Some(glutin::event::VirtualKeyCode::LShift) => {
                                    shift_pressed = false;
                                },
                                _ => (),
                            }
                        },
                    }
                },
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => {}
            },
            _ => (),
        }

        let mut camera_direction = Vector3d::new(
            (vertical_angle.cos() * horizontal_angle.sin()),
            vertical_angle.sin(),
            (vertical_angle.cos() * horizontal_angle.cos()),
        );

        let right = Vector3d::new(0.0, 1.0, 0.0).cross(camera_direction).normalize();
        let up = camera_direction.cross(right).normalize();

        if w_pressed {
            camera_position += camera_direction.bird_view_up().scale(delta_time.as_secs_f32() * 0.5)
        }
        if s_pressed {
            camera_position -= camera_direction.bird_view_up().scale(delta_time.as_secs_f32() * 0.5)
        }
        if a_pressed {
            camera_position += right.bird_view_up().scale(delta_time.as_secs_f32() * 0.5)
        }
        if d_pressed {
            camera_position -= right.bird_view_up().scale(delta_time.as_secs_f32() * 0.5)
        }
        if space_pressed {
            camera_position.y += delta_time.as_secs_f32() * 0.5
        }
        if shift_pressed {
            camera_position.y -= delta_time.as_secs_f32() * 0.5
        }

        view = Matrix44d::look_at(
            camera_position,
            camera_position + camera_direction,
            up,
        );

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniform! {
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            projection: projection.to_list(),
            view: view.to_list(),
            model: model.to_list(),
        }, &Default::default()).unwrap();

        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        time = Instant::now();
    });
}
