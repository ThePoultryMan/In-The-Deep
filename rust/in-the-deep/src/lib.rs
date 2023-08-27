use godot::{engine::Area3D, prelude::*};

mod enemies;
mod player;
mod systems;
mod weapons;

struct InTheDeep;

#[derive(Debug, Clone)]
pub struct RaycastError;

#[gdextension]
unsafe impl ExtensionLibrary for InTheDeep {}

pub fn get_collider_from_raycast(ray_result: Dictionary) -> Result<Gd<Area3D>, RaycastError> {
    let collider_entry = ray_result.get("collider");
    if let Some(collider) = collider_entry {
        let collider_node: Option<Gd<Area3D>> = collider.to();
        if let Some(collider_node) = collider_node {
            return Ok(collider_node);
        }
    }
    Err(RaycastError)
}
