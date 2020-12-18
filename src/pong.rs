use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::transform::Transform,
  ecs::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Pong;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
  let mut transform = Transform::default();
  // Set the camera position
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

  world
      .create_entity()
      .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
      .with(transform)
      .build();
}

impl SimpleState for Pong {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

  }
  
}