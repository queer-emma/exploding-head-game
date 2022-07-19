use legion::system;

use crate::game::{
    components::{
        Dimension,
        GlobalTransform,
        LocalTransform,
        Parent,
        Sprite,
    },
    resources::CurrentCamera,
};

/// updates global transforms.
#[system(for_each)]
fn update_local_transforms(
    global_transform: &mut GlobalTransform,
    local_transform: &LocalTransform,
    parent: &Option<Parent>,
) {
    log::debug!("system running: update local transforms");
}

/// system for rendering sprites from the global sprite sheet.
///
/// todo: move this into graphics. but we can refactor stuff out of the graphics
/// object first.
#[system(for_each)]
fn render_sprites(
    sprite: &mut Sprite,
    dimension: &Dimension,
    global_transform: &GlobalTransform,
    #[resource] current_camera: &CurrentCamera,
) {
    log::debug!("system running: render sprites");

    // todo
}
