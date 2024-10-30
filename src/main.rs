use include_pipeline::{gradient_triangle::run_shader, parse_wgsl_shader};

fn main() {
    let shader_as_str = include_str!("../assets/shaders/gradient_triangle.wgsl");

    parse_wgsl_shader(&shader_as_str);

    run_shader(&shader_as_str);
}