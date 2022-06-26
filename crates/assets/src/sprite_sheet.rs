use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};
use euclid::default::Rect;


#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite {
    pub rect: Rect<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub sprites: HashMap<PathBuf, Sprite>,
}


