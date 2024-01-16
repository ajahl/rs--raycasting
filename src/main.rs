extern crate cgmath;
extern crate gl;
extern crate glfw;

mod camera;
use camera::Camera;

use cgmath::Point2;

use glfw::fail_on_errors;

use glfw::ffi::glfwGetTime;
use glfw::{Action, Context, Key};

use std::f32::consts::PI;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 480;
static TITLE: &str = "Raycasting Showcase";

static MAP: &'static str = "
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

fn make_map(map: &str) {
    let mut result: Vec<Segment> = Vec::new();
    let lines: Vec<&str> = map.split("\n").collect();

    let mut y = lines.len();

    for line in lines {
        let mut x = 0;
        for char in line.chars() {
            // match char {
            //     '#' | '*' => result.append(box(Point2(x, y)));
            //     '/' => result += ul_triangle(Point2(x, y));
            //     '&' => result += ur_triangle(Point2(x, y));
            //     '%' => result += lr_triangle(Point2(x, y));
            //     '`' => result += ll_triangle(Point2(x, y));
            // }
            x += 1
        }
        y -= 1
    }
}

// fn box(point: Point2) {
//     [
//         Segment(point + Point2(0, 0), point + Point2(1, 0)),
//         Segment(point + Point2(1, 0), point + Point2(1, -1)),
//         Segment(point + Point2(0, 0), point + Point2(0, -1)),
//         Segment(point + Point2(0, -1), point + Point2(1, -1)),
//     ]
// }

struct Segment {
    start: Point2<f32>,
    end: Point2<f32>,
}

impl Segment {
    fn new(start: Point2<f32>, end: Point2<f32>) -> Segment {
        Segment { start, end }
    }
}

fn main() {
    let map_wall_segments = make_map(MAP);

    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let fov = calculate_fov(WIDTH);

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
                _ => {}
            }
        }

        for ray in camera.rays(WIDTH) {
            let matches = intersect_ray(ray, map_wall_segments);
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

fn intersect_ray(ray: (camera::Ray, Point2<f32>), map_wall_segments: ()) -> () {
    todo!()
}

fn calculate_fov(width: u32) -> f32 {
    let width = width as f32;
    let half_fov_rad = (PI / 2.0) / 2.0;
    2.0 * (width / 800.0 * half_fov_rad.tan()).atan()
}
