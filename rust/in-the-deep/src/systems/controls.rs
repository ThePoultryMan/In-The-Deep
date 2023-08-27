use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct InputManager {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for InputManager {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let velocity = input.get_vector(
            StringName::from("move_left"),
            StringName::from("move_right"),
            StringName::from("move_forward"),
            StringName::from("move_back"),
        );

        if !velocity.is_zero_approx() {
            self.base.emit_signal("input_move".into(), &[velocity.to_variant()]);
        }

        if input.is_action_just_pressed(StringName::from("primary")) {
            self.base.emit_signal("primary".into(), &[]);
        }
    }
}

#[godot_api]
impl InputManager {
    #[signal]
    fn input_move(velocity: Vector2) {}

    #[signal]
    fn primary() {}
}
