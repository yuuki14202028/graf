mod vertex;
mod matrix44d;
mod vector3d;

#[macro_use]
extern crate glium;

use std::f32::consts::PI;
use std::time::Instant;
use glium::{glutin, Surface};
use crate::matrix44d::Matrix44d;
use crate::vector3d::Vector3d;
use crate::vertex::Vertex;
use std::{env, fs};
use std::path::Path;
use glium::glutin::event::{ElementState, MouseButton};
use glium::index::PrimitiveType::TriangleStrip;


fn main() -> Result<(), std::io::Error> {

    let path = env::current_dir()?;
    println!("starting dir: {}", path.display());

    let vert_str = fs::read_to_string("./shader/default.vert")?;
    let frag_str = fs::read_to_string("./shader/default.frag")?;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [   0.5,  0.5,  0.5], normal: [  0.5,  0.5,  0.5] };
    let vertex2 = Vertex { position: [  -0.5,  0.5,  0.5], normal: [ -0.5,  0.5,  0.5] };
    let vertex3 = Vertex { position: [  -0.5,  0.5, -0.5], normal: [ -0.5,  0.5, -0.5] };
    let vertex4 = Vertex { position: [   0.5,  0.5, -0.5], normal: [  0.5,  0.5, -0.5] };
    let vertex5 = Vertex { position: [   0.5, -0.5,  0.5], normal: [  0.5, -0.5,  0.5] };
    let vertex6 = Vertex { position: [  -0.5, -0.5,  0.5], normal: [ -0.5, -0.5,  0.5] };
    let vertex7 = Vertex { position: [  -0.5, -0.5, -0.5], normal: [ -0.5, -0.5, -0.5] };
    let vertex8 = Vertex { position: [   0.5, -0.5, -0.5], normal: [  0.5, -0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6, vertex7, vertex8];

    let vertex1_mini = Vertex { position: [   0.05,  0.05,  0.05], normal: [  0.05,  0.05,  0.05] };
    let vertex2_mini = Vertex { position: [  -0.05,  0.05,  0.05], normal: [ -0.05,  0.05,  0.05] };
    let vertex3_mini = Vertex { position: [  -0.05,  0.05, -0.05], normal: [ -0.05,  0.05, -0.05] };
    let vertex4_mini = Vertex { position: [   0.05,  0.05, -0.05], normal: [  0.05,  0.05, -0.05] };
    let vertex5_mini = Vertex { position: [   0.05, -0.05,  0.05], normal: [  0.05, -0.05,  0.05] };
    let vertex6_mini = Vertex { position: [  -0.05, -0.05,  0.05], normal: [ -0.05, -0.05,  0.05] };
    let vertex7_mini = Vertex { position: [  -0.05, -0.05, -0.05], normal: [ -0.05, -0.05, -0.05] };
    let vertex8_mini = Vertex { position: [   0.05, -0.05, -0.05], normal: [  0.05, -0.05, -0.05] };
    let shape_mini = vec![vertex1_mini, vertex2_mini, vertex3_mini, vertex4_mini, vertex5_mini, vertex6_mini, vertex7_mini, vertex8_mini];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let vertex_buffer_mini = glium::VertexBuffer::new(&display, &shape_mini).unwrap();

    let indices: [u16; 36] = [
        // Ceiling
        0, 1, 2,
        2, 3, 0,

        // Floor
        4, 5, 6,
        6, 7, 4,

        0, 1, 5,
        5, 4, 0,

        1, 2, 6,
        6, 5, 1,

        2, 3, 7,
        7, 6, 2,

        3, 0, 4,
        4, 7, 3
    ];
    let index_buffer = glium::IndexBuffer::new(&display, TriangleStrip, &indices).unwrap();

    let light = [-1.0, 0.4, 0.9f32];

    let mut camera_position = Vector3d::new(0.0, 0.0, 1.0);

    let mut horizontal_angle: f32 = PI;
    let mut vertical_angle: f32 = 0.0;

    let mut right_pressed = false;
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
    let model = Matrix44d::translate(0.0, 0.0, 0.0);

    let program = glium::Program::from_source(&display, &vert_str, &frag_str, None).unwrap();

    let mut time = std::time::Instant::now();
    let mut ticks: f32 = 0.0;
    let mut mouse_move = false;
    let mut mouse_delta = (0.0, 0.0);

    event_loop.run(move |ev, _, control_flow| {
        mouse_move = false;
        match ev {
            glutin::event::Event::DeviceEvent {event, ..} => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    mouse_delta.0 = delta.0 as f32 / 500.0;
                    mouse_delta.1 = delta.1 as f32 / 500.0;
                    mouse_move = true;
                },
                _ => {}
            }
            glutin::event::Event::WindowEvent {event , ..} => match event {
                glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                    match state {
                        ElementState::Pressed => {
                            match button {
                                MouseButton::Right => {
                                    right_pressed = true;
                                }
                                _ => {}
                            }
                        }
                        ElementState::Released => {
                            match button {
                                MouseButton::Right => {
                                    right_pressed = false;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    match input.state {
                        ElementState::Pressed => {
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
                        ElementState::Released => {
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

        if right_pressed && mouse_move {
            horizontal_angle -= mouse_delta.0;
            vertical_angle -= mouse_delta.1;
        }

        let mut camera_direction = Vector3d::new(
            (vertical_angle.cos() * horizontal_angle.sin()),
            vertical_angle.sin(),
            (vertical_angle.cos() * horizontal_angle.cos()),
        );

        let right = Vector3d::new(0.0, 1.0, 0.0).cross(camera_direction).normalize();
        let up = camera_direction.cross(right).normalize();
        let start_time = Instant::now();
        let delta_time = start_time.duration_since(time);
        time = Instant::now();

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
        target.clear_color_and_depth((0.025, 0.025, 0.025, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(&vertex_buffer, &index_buffer, &program, &uniform! {
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            projection: projection.to_list(),
            view: view.to_list(),
            model: model.to_list(),
            u_light: light,
            pos: Vector3d::new(0.0, 0.0, 0.0).to_list()
        }, &params).unwrap();

        println!("{:?}", camera_position);

        target.draw(&vertex_buffer_mini, &index_buffer, &program, &uniform! {
            resolution: [display.get_framebuffer_dimensions().0 as f32, display.get_framebuffer_dimensions().1 as f32],
            projection: projection.to_list(),
            view: view.to_list(),
            model: model.to_list(),
            u_light: light,
            pos: (camera_position + camera_direction).to_list()
        }, &params).unwrap();

        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
