extern crate cgmath;
extern crate gl;
extern crate glfw;

mod camera;
mod line;
mod map;
mod ray;
mod segment;

use camera::Camera;
use cgmath::Point2;
use glfw::fail_on_errors;
use glfw::ffi::glfwGetTime;
use glfw::{Action, Context, Key};

use map::Map;
use ray::Ray;

use std::f32::consts::PI;

static TITLE: &str = "Raycasting Showcase";

static WIDTH: u32 = 800;
static HEIGHT: u32 = 480;

static MAP: &str = "
###########`&#######
#           ` / /  #
#/%#/&`&/&`& % `%`&#
# / %  / `/% &  /  #
#& / `   & / & /%/%#
# `&  & `& ` `% ` &#
#  % # / `%&  # `& #
#% /% %`` / %/& &  #
#/% /   &`%/ % /%& #
# # //&    %& %`&  #
#  % %`  %/     % &#
####################
";

fn main() {
    let map_wall_segments = Map::make_map(MAP);

    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let fov = Camera::calculate_fov(WIDTH);

    let camera = Camera::new(Point2::new(-0.5, -0.5), PI / 2.0, fov);

    let mut last_frame_time = unsafe { glfwGetTime() };

    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Pos(_, _) => todo!(),
                glfw::WindowEvent::Size(_, _) => todo!(),
                glfw::WindowEvent::Close => todo!(),
                glfw::WindowEvent::Refresh => todo!(),
                glfw::WindowEvent::Focus(_) => todo!(),
                glfw::WindowEvent::Iconify(_) => todo!(),
                glfw::WindowEvent::FramebufferSize(_, _) => todo!(),
                glfw::WindowEvent::MouseButton(_, _, _) => todo!(),
                glfw::WindowEvent::CursorPos(_, _) => todo!(),
                glfw::WindowEvent::CursorEnter(_) => todo!(),
                glfw::WindowEvent::Scroll(_, _) => todo!(),
                glfw::WindowEvent::Char(_) => todo!(),
                glfw::WindowEvent::CharModifiers(_, _) => todo!(),
                glfw::WindowEvent::FileDrop(_) => todo!(),
                glfw::WindowEvent::Maximize(_) => todo!(),
                glfw::WindowEvent::ContentScale(_, _) => todo!(),
            }
        }

        for (ray, segment_point) in camera.rays(WIDTH) {
            let matches = Ray::intersect(ray, &map_wall_segments);
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let current = glfwGetTime();
            let delta = current - last_frame_time;
            last_frame_time = current;
            // std::cout << 1.f / delta << std::endl;
            let fps = 1.0 / delta;
            window.set_title(&format!("{} - FPS: {:.0}", TITLE, fps));
        }

        window.swap_buffers();
    }
}
