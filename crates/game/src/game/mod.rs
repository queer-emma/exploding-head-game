//! # todo
//! 
//! create components for camera and sprite.

use legion::World;
use nalgebra::{Point2, Vector2};

use crate::graphics::Graphics;



/// dimension of the object.
pub enum Dimension {
    /// a.k.a. wonderworld. this is the imagined world inside the player's head.
    InnerWorld,

    /// a.k.a. reality. the physical game world. represents shared reality.
    OuterWorld,
}





/// position in the world
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position(Point2<f32>);

/// velocity of an object. this will be added to the position in every tick (asjusted for framerate)
#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity(Vector2<f32>);


struct Acceleration(Vector2<f32>);





#[derive(Debug)]
struct Game {
    /// the game world that contains all entities
    world: World,

    // todo: put graphics stuff in here too.
    graphics: Graphics,
}



struct Camera {
    // todo
}

/// todo: something better than a plain index would be nice.
struct Sprite {
    sprite_index: usize,
}