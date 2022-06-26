/*
use std::{io::{
    Cursor,
    Read,
}, collections::HashMap, path::Path};

use utils::atlas::SpriteSheet;

use guillotiere::{
    AllocId,
    Allocation,
    AtlasAllocator,
    ChangeList,
    Size,
};
use image::{
    io::Reader,
    GenericImage,
    RgbaImage,
};
use murmur3::{
    murmur3_x64_128,
};
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Image(#[from] image::ImageError),

    #[error("Allocations failed")]
    AllocationsFailed(Vec<Allocation>),

    #[error("Texture size mismatch")]
    TextureSizeMismatch,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Padding {
    None,
    Tiled,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TextureId(u128);

/// # todo
///
///  - padding[1]
///  - pack into multiple atlas textures[2]
///
/// [1]: https://stackoverflow.com/questions/16050574/hlsl-color-bleeding-when-using-a-texture-atlas
/// [2]: https://github.com/chinedufn/rectangle-pack - use [`Padding`] enum to determine how we pad the image in the atlas.
/// 
pub struct AtlasBuilder {
    /// textures
    textures: HashMap<TextureId, RgbaImage>,

    allocations: HashMap<TextureId, Allocation>,

    /// The allocator that arranges where to put textures in the atlas.
    allocator: AtlasAllocator,

    /// Whether the builder was modified since the last rearrange.
    dirty: bool,
}

impl Default for AtlasBuilder {
    fn default() -> Self {
        let allocator = AtlasAllocator::new(Size::from([1024, 1024]));

        AtlasBuilder {
            textures: HashMap::new(),
            allocations: HashMap::new(),
            allocator,
            dirty: false,
        }
    }
}

impl AtlasBuilder {
    pub const HASH_SEED: u32 = 1312;

    pub fn push_image_from_slice(
        &mut self,
        data: &[u8],
        mut size: Option<[i32; 2]>,
        _padding: Padding,
    ) -> Result<MaterialId, Error> {
        let hash = murmur3_x64_128(data, Self::HASH_SEED);

        if let Some(texture_id) = self.textures.get(&hash) {
            return Ok(*texture_id);
        }
        else {
            let image = Reader::new(Cursor::new(buf))
                .with_guessed_format()?
                .decode()?;
            let image = image.into_rgba8();
            if size.is_none() {
                size = Some([image.width() as i32, image.height() as i32]);
            }
            self.textures.insert(hash, image);
        }

        let size = size.expect("size");

        // try allocating. if it doesn't fit, grow the texture atlas.
        let allocation = loop {
            if let Some(allocation) = self.allocator.allocate(Size::new(size[0], size[1])) {
                break allocation;
            }
            else {
                self.grow()?;
            }
        };

        self.allocations.insert(TextureId(hash), allocation);

        Ok(TextureId(hash))
    }

    fn grow(&mut self) -> Result<(), Error> {
        let new_size = self.allocator.size() * 2;
        log::debug!("Resizing texture atlas to: {:?}", new_size);

        let change_list = self.allocator.resize_and_rearrange(new_size);
        self.apply_change_list(&change_list)?;

        Ok(())
    }

    fn rearrange_if_dirty(&mut self) -> Result<(), Error> {
        if self.dirty {
            let change_list = self.allocator.rearrange();
            self.apply_change_list(&change_list)?;
            self.dirty = false;
        }

        Ok(())
    }

    fn apply_change_list(&mut self, change_list: &ChangeList) -> Result<(), Error> {
        if !change_list.failures.is_empty() {
            return Err(Error::AllocationsFailed(change_list.failures.clone()));
        }

        let changes = change_list
            .changes
            .iter()
            .map(|change| (change.old.id, change.new.id))
            .collect::<AHashMap<AllocId, AllocId>>();

        for (_, (_, allocation)) in self.materials.iter_mut() {
            if let Some(new_id) = changes.get(&allocation.id) {
                allocation.id = *new_id;
            }
        }

        Ok(())
    }

    pub fn build(mut self) -> Result<SpriteSheet<u32>, Error> {
        self.rearrange_if_dirty()?;

        let size = self.allocator.size();
        let mut atlas_texture = RgbaImage::new(size.width as u32, size.height as u32);
        let mut atlas = HashMap::new();

        for (teture_id, image) in &self.textures {
            let allocation = self.allocations.get(texture_id).unwrap();

            let mut target = atlas_texture.sub_image(
                allocation.rectangle.min.x as u32,
                allocation.rectangle.min.y as u32,
                allocation.rectangle.width() as u32,
                allocation.rectangle.height() as u32,
            );

            target.copy_from(image, 0, 0).unwrap();

            atlas.insert(hash, allocation.rectangle.to_rect());
        }
        
        self.textures.shrink_to_fit();

        Ok(SpriteSheet { sprites: self.textures })
    }
}
*/


use std::path::Path;
use color_eyre::eyre::Error;

pub async fn build<P: AsRef<Path>>(output_texture: impl AsRef<Path>, output_sprite_sheet: impl AsRef<Path>, files: &[P]) -> Result<(), Error> {
    todo!();
}
