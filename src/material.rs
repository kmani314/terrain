extern crate kiss3d;
extern crate gl;
extern crate nalgebra as na;

use std::ptr; 
use gl::types::GLint; 
use na::{Point2, Point3, Vector3, Matrix3, Matrix4, Isometry3}; 
use kiss3d::scene::ObjectData; 
use kiss3d::camera::Camera; 
use kiss3d::light::Light; 
use kiss3d::resource::{Shader, ShaderUniform, ShaderAttribute, Material, Mesh};

pub struct Color(pub f32, pub f32, pub f32);

// Terrain Material
// Copy of the default kiss3d::builtin::ObjectMaterial with some additions for height based colors
pub struct TerrainMaterial {
    shader: Shader,
    position: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    ntransform: ShaderUniform<Matrix3<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
    light: ShaderUniform<Point3<f32>>,
    color: ShaderUniform<Point3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
}

impl TerrainMaterial {
    pub fn new(/*thresholds: &Vec<f32>, colors: Vec<Color>*/) -> TerrainMaterial {
        let mut shader = Shader::new_from_str(VSHADER, FSHADER);
        shader.use_program();
        
        TerrainMaterial {
            position: shader.get_attrib("position").unwrap(),
            normal: shader.get_attrib("normal").unwrap(),
            view: shader.get_uniform("view").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            ntransform: shader.get_uniform("ntransform").unwrap(),
            scale: shader.get_uniform("scale").unwrap(),
            light: shader.get_uniform("light_position").unwrap(),
            color: shader.get_uniform("color").unwrap(),
            tex_coord: shader.get_attrib("tex_coord_v").unwrap(),
            shader,
        }
    }
    
    fn activate(&mut self) {
        self.shader.use_program();
        self.position.enable();
        self.normal.enable();
        self.tex_coord.enable();
    }
    
    fn deactivate(&mut self) {
        self.position.disable();
        self.normal.disable();
        self.tex_coord.disable();
    }
}

impl Material for TerrainMaterial {
    fn render(&mut self, 
        pass: usize, 
        transform: &Isometry3<f32>, 
        scale: &Vector3<f32>, 
        camera: &mut dyn Camera, 
        light: &Light,
        data: &ObjectData, // needed for color rendering
        mesh: &mut Mesh) {

        self.activate();

        camera.upload(pass, &mut self.view);

        let pos = match *light {
            Light::Absolute(ref p) => *p,
            Light::StickToCamera  => camera.eye()
        };
        
        self.light.upload(&pos);

        let formated_transform = transform.to_homogeneous();
        let formated_ntransform = transform.rotation.to_rotation_matrix().into_inner();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        unsafe {
            self.transform.upload(&formated_transform);
            self.ntransform.upload(&formated_ntransform);
            self.scale.upload(&formated_scale);
            self.color.upload(data.color());

            mesh.bind(&mut self.position, &mut self.normal, &mut self.tex_coord);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, data.texture().id());

            gl::DrawElements(gl::TRIANGLES, mesh.num_pts() as GLint, gl::UNSIGNED_INT, ptr::null());
        }

        mesh.unbind();
        self.deactivate();
    }
}

static VSHADER: &str =
"#version 120
attribute vec3 position;
attribute vec3 normal;
attribute vec3 color;
attribute vec2 tex_coord_v;
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
}";

static FSHADER: &str =
"#version 120
uniform vec3      color;
uniform vec3      light_position;
uniform sampler2D tex;
varying vec2      tex_coord;
varying vec3      ws_normal;
varying vec3      ws_position;
void main() {
  vec3 L = normalize(light_position - ws_position);
  vec3 E = normalize(-ws_position);

  // Height Based Colors:

  //calculate Ambient Term:
  vec4 Iamb = vec4(color, 1.0);

  //calculate Diffuse Term:
  vec4 Idiff1 = vec4(1.0, 1.0, 1.0, 1.0) * max(dot(ws_normal,L), 0.0);
  Idiff1 = clamp(Idiff1, 0.0, 1.0);

  // double sided lighting:
  vec4 Idiff2 = vec4(1.0, 1.0, 1.0, 1.0) * max(dot(-ws_normal,L), 0.0);
  Idiff2 = clamp(Idiff2, 0.0, 1.0);

  vec4 tex_color = texture2D(tex, tex_coord);
  gl_FragColor   = tex_color * (Iamb + (Idiff1 + Idiff2) / 2) / 2;
}";
