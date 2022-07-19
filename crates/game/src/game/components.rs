use nalgebra::{
    Isometry3,
    Perspective3,
    Point2,
    Point3,
    Similarity3,
    Vector2,
    Vector3,
};

/// dimension of the object.
pub enum Dimension {
    /// a.k.a. wonderworld. this is the imagined world inside the player's head.
    InnerWorld,

    /// a.k.a. reality. the physical game world. represents shared reality.
    OuterWorld,
}

/// a model's transform relative to the parent
///
/// todo: should we just add the parent entity to this?
#[derive(Clone, Copy, Debug)]
pub struct LocalTransform(pub Similarity3<f32>);

impl Default for LocalTransform {
    fn default() -> Self {
        Self(Similarity3::new(
            nalgebra::zero(),
            nalgebra::zero(),
            nalgebra::one(),
        ))
    }
}

impl LocalTransform {
    pub fn new(x: impl Into<Similarity3<f32>>) -> Self {
        Self(x.into())
    }

    /// todo: call this from a system to update the global transform. or remove
    /// this method and move the code altogether.
    pub fn to_global<'a>(
        self,
        parent_transform: impl Into<Option<&'a GlobalTransform>>,
    ) -> GlobalTransform {
        if let Some(parent_transform) = parent_transform.into() {
            GlobalTransform(&parent_transform.0 * &self.0)
        }
        else {
            GlobalTransform(self.0)
        }
    }
}

/// a model's global transform
#[derive(Clone, Copy, Debug)]
pub struct GlobalTransform(pub Similarity3<f32>);

impl GlobalTransform {
    pub fn new(similarity: impl Into<Similarity3<f32>>) -> Self {
        Self(similarity.into())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parent {
    // todo: entity id of parent. we'll use this when we compute the absolute transform for an
    // entity.
}

/// position in the world
#[derive(Clone, Copy, Debug)]
pub struct Position(pub Point2<f32>);

/// orientation of entity. the vector points up.
#[derive(Clone, Copy, Debug)]
pub struct Orientation(pub Vector2<f32>);

/// velocity of an object. this will be added to the position in every tick
/// (asjusted for framerate)
#[derive(Clone, Copy, Debug)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Clone, Copy, Debug)]
pub struct Acceleration(pub Vector2<f32>);

/// camera component. the camera entity will also need a position and
/// orientation component (or use defaults). we can compute.
///
/// todo: we want projection and not orthographic, right? with perspective we
/// can have parallex with the background.
pub struct Camera {
    camera_projection: Perspective3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 9.0, std::f32::consts::FRAC_PI_2, 1.0, 1000.0)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
        // our camera looks toward the point (0.0, 0.0, 0.0).
        // it is located at (0.0, 0.0, 1.0).
        let eye = Point3::from([0.0, 0.0, 1.0]); // todo: this is the global transform of the camera entity

        let target = Point3::from([0.0, 0.0, 0.0]);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        // a perspective projection.
        // todo: use the aspect ratio from the window
        let camera_projection = Perspective3::new(aspect_ratio, fovy, znear, zfar);

        Self { camera_projection }
    }
}

/// component that hides entities.
pub struct Hidden;

/// todo: something better than a plain index would be nice.
pub struct Sprite {
    sprite_index: usize,
}

impl Sprite {
    pub fn new(sprite_index: usize) -> Self {
        Self { sprite_index }
    }
}
