use std::time::{
    Duration,
};

use instant::Instant;
use legion::Entity;

/// this defines which camera is used for rendering. this also currently
/// restricts stuff to rendering to exactly one surface, which is not strictly
/// necessary.
///
/// the dimension attached to the camera will determine which entities are
/// endered.
///
/// # todo
///
/// - add render target, which is basically the wgpu surface and other stuff.
#[derive(Copy, Clone, Debug)]
pub struct CurrentCamera {
    pub entity: Entity,
}

/// resource with main loop timing information. this is updated when rendering
/// is requested.
pub struct Time {
    last_render: Instant,
    since_last_render: Duration,

    fps_next: Instant,
    fps_counter: u32,
    fps: u32,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            last_render: now,
            since_last_render: Duration::new(0, 0),
            fps_next: now,
            fps_counter: 0,
            fps: 0,
        }
    }
}

impl Time {
    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub(super) fn update(&mut self) {
        let now = Instant::now();
        self.since_last_render = now - self.last_render;
        self.last_render = now;

        if now >= self.fps_next {
            self.fps_next = now + Duration::new(1, 0);
            self.fps = self.fps_counter;
            self.fps_counter = 0;
            log::debug!("fps: {}", self.fps);
        }
        else {
            // is this exact enough?
            self.fps_counter += 1;
        }
    }
}
