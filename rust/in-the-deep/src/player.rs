use godot::{
    engine::{CharacterBody3D, CharacterBody3DVirtual, InputEvent, input::MouseMode, InputEventMouseMotion},
    prelude::*,
};

use crate::systems::controls::InputManager;

// const GRAVITY: real = -0.05;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    #[export]
    input_manager: Option<Gd<InputManager>>,

    #[export]
    neck: Option<Gd<Node3D>>,
    #[export]
    camera: Option<Gd<Camera3D>>,

    #[base]
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl CharacterBody3DVirtual for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            input_manager: None,
            neck: None,
            camera: None,
            base,
        }
    }

    fn ready(&mut self) {
        if let Some(input_manager) = &mut self.input_manager {
            input_manager.connect(
                "input_move".into(),
                Callable::from_object_method(self.base.get_node_as::<Self>("."), "move_input"),
            );
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let mut input = Input::singleton();
        if event.is_class("InputEventMouseButton".into()) {
            input.set_mouse_mode(MouseMode::MOUSE_MODE_CAPTURED);
        } else if event.is_action_pressed("ui_cancel".into()) {
            input.set_mouse_mode(MouseMode::MOUSE_MODE_VISIBLE);
        }

        if input.get_mouse_mode() == MouseMode::MOUSE_MODE_CAPTURED {
            if event.is_class("InputEventMouseMotion".into()) {
                if let (Some(camera), Some(neck)) = (&mut self.camera, &mut self.neck) {
                    let mouse_motion: Gd<InputEventMouseMotion> = event.cast();
                    camera.rotate_x(-mouse_motion.get_relative().y * 0.01);
                    neck.rotate_y(-mouse_motion.get_relative().x * 0.01);
                }
            }
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    fn move_input(&mut self, direction: Vector2) {
        if let Some(neck) = &self.neck {
            let mut movement_direction: Vector3 = Vector3::new(direction.x * 0.1, 0.0, direction.y * 0.1);
            movement_direction = neck.get_transform().basis * movement_direction;
            self.base.move_and_collide(movement_direction);
        }
    }
}
