use godot::engine::{AnimatedSprite2D, AnimatedSprite2DVirtual, PhysicsRayQueryParameters3D};
use godot::prelude::*;
use macros::enemy_node_from_class;

use crate::enemies::owen::Owen;
use crate::systems::controls::InputManager;

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Gun {
    #[export]
    damage: real,
    #[export]
    can_shoot: bool,

    #[export]
    shoot_point: Option<Gd<Node3D>>,
    #[export]
    input_manager: Option<Gd<InputManager>>,

    #[base]
    base: Base<AnimatedSprite2D>,
}

#[godot_api]
impl AnimatedSprite2DVirtual for Gun {
    fn init(base: Base<AnimatedSprite2D>) -> Self {
        Gun {
            damage: 10.0,
            can_shoot: true,
            shoot_point: None,
            input_manager: None,
            base,
        }
    }

    fn ready(&mut self) {
        if let Some(input_manager) = &mut self.input_manager {
            input_manager.connect(
                "primary".into(),
                Callable::from_object_method(self.base.get_node_as::<Self>("."), "shoot"),
            );
        }

        let play_idle_animation = self.base.get_node_as::<Self>(".");
        self.base.connect(
            "animation_finished".into(),
            Callable::from_object_method(play_idle_animation, "idle"),
        );
    }
}

#[godot_api]
impl Gun {
    #[func]
    fn shoot(&mut self) {
        if self.can_shoot {
            self.base.set_animation("shoot".into());
            self.base.play();

            if let Some(shoot_point) = &self.shoot_point {
                let mut space_state = shoot_point.get_world_3d().unwrap().get_direct_space_state().unwrap();
                let mut raycast = PhysicsRayQueryParameters3D::create(shoot_point.get_global_position(), Vector3 { x: 0.0, y: 0.0, z: 5.0 }).unwrap();
                raycast.set_collide_with_areas(true);
                let collider = crate::get_collider_from_raycast(space_state.intersect_ray(raycast));
                let mut collider_area = None;
                match collider {
                    Ok(area) => collider_area = Some(area),
                    Err(_) => godot_error!("Error occurred while raycasting gun."),
                }

                if let Some(collider_area) = collider_area {
                    if let Some(enemy) = collider_area.get_parent() {
                        let mut typed_enemy = enemy_node_from_class!(enemy, Owen);
                        typed_enemy.call("p_damage".into(), &[self.damage.to_variant()]);
                    }
                }
            }

            self.set_can_shoot(false);
        }
    }

    #[func]
    fn idle(&mut self) {
        self.base.set_animation("idle".into());
        self.set_can_shoot(true);
    }
}
