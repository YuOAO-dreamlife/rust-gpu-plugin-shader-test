#![no_std]
#![feature(asm_experimental_arch)]

pub use bevy_pbr_rust::prelude::*;

#[warn(unused_imports)]
use spirv_std::num_traits::Float;

use spirv_std::{
    glam::{vec2, vec3, Vec2, Vec3, Vec4},
    spirv,
};

fn oklab_to_linear_srgb(c: Vec3) -> Vec3 {
    let L = c.x;
    let a = c.y;
    let b = c.z;

    let l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    return vec3(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}

#[spirv(vertex)]
pub fn vertex_warp(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] view: &View,
    in_position: Vec4,
    in_normal: Vec3,
    in_uv: Vec2,

    #[spirv(position)] out_clip_position: &mut Vec4,
    out_world_position: &mut Vec4,
    out_world_normal: &mut Vec3,
    out_uv: &mut Vec2,
) {
    *out_clip_position = view.mesh_position_world_to_clip(in_position);

    *out_world_position = in_position;
    *out_world_normal = in_normal;
    *out_uv = in_uv;
}

#[spirv(fragment)]
#[allow(unused_variables)]
pub fn fragment_normal(
    in_world_position: Vec4,
    in_world_normal: Vec3,
    in_uv: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 9)] globals: &Globals,
    out_color: &mut Vec4,
) {
    let speed = 2.0;
    let t_1 = (globals.time * speed).sin() * 0.5 + 0.5;
    let t_2 = (globals.time * speed).cos();

    let distance_to_center = Vec2::distance(in_uv, vec2(0.5, 0.5)) * 1.4;

    // 使用該網址的色彩呈現方式 https://bottosson.github.io/posts/oklab/
    let red = vec3(0.627955, 0.224863, 0.125846);
    let green = vec3(0.86644, -0.233887, 0.179498);
    let blue = vec3(0.701674, 0.274566, -0.169156);
    let white = vec3(1.0, 0.0, 0.0);
    let mixed = Vec3::lerp(
        Vec3::lerp(red, blue, t_1),
        Vec3::lerp(green, white, t_2),
        distance_to_center,
    );

    *out_color = oklab_to_linear_srgb(mixed).extend(1.0);
}

#[spirv(vertex)]
pub fn vertex_warp_colorful(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] view: &View,

    in_position: Vec4,
    in_normal: Vec3,
    in_uv: Vec2,

    #[spirv(position)] out_clip_position: &mut Vec4,
    out_world_position: &mut Vec4,
    out_world_normal: &mut Vec3,
    out_uv: &mut Vec2,
) {
    *out_clip_position = view.mesh_position_world_to_clip(in_position);

    *out_world_position = in_position;
    *out_world_normal = in_normal;
    *out_uv = in_uv;
}

#[spirv(fragment)]
#[allow(unused_variables)]
pub fn fragment_normal_colorful(
    in_world_position: Vec4,
    in_world_normal: Vec3,
    in_uv: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 9)] globals: &Globals,
    out_color: &mut Vec4,
) {
    *out_color = in_world_position
    // *out_color = in_world_normal.extend(1.0)
    // *out_color = in_uv.extend(0.0).extend(0.0)
}
