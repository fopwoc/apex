use cgmath::{Matrix4, Vector3, Quaternion};

pub trait Transformation {
    fn apply(&self) -> Matrix4<f32>;
}

// Camera
pub trait Camera: Transformation {
    fn get_position(&self) -> Vector3<f32>;
    fn get_scale(&mut self) -> Vector3<f32>;
    fn get_rotation(&self) -> Quaternion<f32>;
    
    fn set_position(&mut self, value: Vector3<f32>);
    fn set_scale(&mut self, value: Vector3<f32>);
    fn set_rotation(&mut self, value: Quaternion<f32>);

    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
    fn set_x(&mut self, value: f32);
    fn set_y(&mut self, value: f32);
    fn set_z(&mut self, value: f32);
}

/* Camera 2D */
pub struct Camera2D {
    position : Vector3<f32>,
    scale    : Vector3<f32>,
    rotation : Quaternion<f32>,
}

impl Camera2D {
    pub fn new<V, Q>(position: V, rotation: Q, scale: V) -> Self
    where
      V: Into<Vector3<f32>>,
      Q: Into<Quaternion<f32>> {
        return Self {
            position : position . into(),
            scale    : scale    .into(),
            rotation : rotation . into(),
        };
    }
}

impl Camera for Camera2D {
    fn get_position(&self) -> Vector3<f32>    { return self.position; }
    fn get_scale(&mut self) -> Vector3<f32>   { return self.scale;    }
    fn get_rotation(&self) -> Quaternion<f32> { return self.rotation  }
    
    fn set_position(&mut self, value: Vector3<f32>)    { self.position = value; }
    fn set_scale(&mut self, value: Vector3<f32>)       { self.scale    = value; }
    fn set_rotation(&mut self, value: Quaternion<f32>) { self.rotation = value; }

    fn get_x(&self) -> f32 { return self.position.x; }
    fn get_y(&self) -> f32 { return self.position.y; }
    fn get_z(&self) -> f32 { return self.position.z; }
    fn set_x(&mut self, value: f32) { self.position.x = value; }
    fn set_y(&mut self, value: f32) { self.position.y = value; }
    fn set_z(&mut self, value: f32) { self.position.z = value; }
}

impl Transformation for Camera2D {
    fn apply(&self) -> Matrix4<f32> {
        let model = cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
                  * cgmath::Matrix4::from_translation(self.position)
                  * cgmath::Matrix4::from(self.rotation);

        return model;
    }
}

// Projection
pub trait Projection: Transformation {
    fn resize(&mut self, width: u32, height: u32);
}

/* Orthographic projection matrix */
pub struct ProjectionOrthographic {
    width  : f32,
    height : f32,
    znear  : f32,
    zfar   : f32,
}

impl ProjectionOrthographic {
    pub fn new(width: u32, height: u32, znear: f32, zfar: f32) -> Self {
        return Self {
            width  : width as f32,
            height : height as f32,
            znear  : znear,
            zfar   : zfar,
        };
    }
}

impl Projection for ProjectionOrthographic {
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
    }
}

impl Transformation for ProjectionOrthographic {
    fn apply(&self) -> Matrix4<f32> {
        return cgmath::ortho(0.0, self.width, self.height, 0.0, self.znear, self.zfar);
    }
}