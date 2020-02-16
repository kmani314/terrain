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
pub struct TerrainMaterial<'a> {
    shader: Shader,
    position: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    ntransform: ShaderUniform<Matrix3<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
    light: ShaderUniform<Point3<f32>>,
    //thresholds: ShaderAttribute<GLfloat>,
    //colors: ShaderAttribute<Vector3<f32>>,
    tex_coord: ShaderAttribute<Point2<f32>>,
    in_thresholds: &'a [f32],
    in_colors: &'a [Color],
}

impl<'a> TerrainMaterial<'a> {
    pub fn new(in_thresholds: &'a [f32], in_colors: &'a [Color]) -> TerrainMaterial<'a> {
        let mut shader = Shader::new_from_str(include_str!("shaders/vector.glsl"), include_str!("shaders/fragment.glsl"));
        shader.use_program();
        
        TerrainMaterial {
            position: shader.get_attrib("position").unwrap(),
            normal: shader.get_attrib("normal").unwrap(),
            view: shader.get_uniform("view").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            ntransform: shader.get_uniform("ntransform").unwrap(),
            scale: shader.get_uniform("scale").unwrap(),
            light: shader.get_uniform("light_position").unwrap(),
            tex_coord: shader.get_attrib("tex_coord_v").unwrap(),
            in_thresholds,
            in_colors,
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

impl<'a> Material for TerrainMaterial<'a> {
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
            //self.color.upload(data.color());

            mesh.bind(&mut self.position, &mut self.normal, &mut self.tex_coord);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, data.texture().id());

            gl::DrawElements(gl::TRIANGLES, mesh.num_pts() as GLint, gl::UNSIGNED_INT, ptr::null());
        }

        mesh.unbind();
        self.deactivate();
    }
}
