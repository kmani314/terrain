#version 120
attribute vec3 position;
attribute vec3 normal;
//attribute vec3[] colors;
attribute vec2 tex_coord_v;
//attribute float[16] thresholds;
//attribute int num_thresh;

varying vec3 ws_normal;
varying vec3 ws_position;
varying vec2 tex_coord;
uniform mat4 view;
uniform mat4 transform;
uniform mat3 scale;
uniform mat3 ntransform;

void main() {
    mat4 scale4 = mat4(scale);
    vec4 pos4   = transform * scale4 * vec4(position, 1.0);
    tex_coord   = tex_coord_v;
    ws_position = pos4.xyz;
    gl_Position = view * pos4;
    ws_normal   = normalize(ntransform * scale * normal);
}
