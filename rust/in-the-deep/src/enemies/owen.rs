use attributes::export_enemy_functions;
use derive::Enemy;
use godot::{prelude::*, engine::{Sprite3D, Sprite3DVirtual}};

use super::{EnemyData, Enemy};

#[derive(GodotClass, Enemy)]
#[class(base=Sprite3D)]
pub struct Owen {
    #[enemy_data]
    enemy: EnemyData,

    #[export]
    player: Option<Gd<Node3D>>,

    #[base]
    base: Base<Sprite3D>
}

#[godot_api]
impl Sprite3DVirtual for Owen {
    fn init(base: Base<Sprite3D>) -> Self {
        Self {
            enemy: EnemyData::with_health(25.0),
            player: None,
            base
        }
    }

    fn ready(&mut self) {
        if self.player.is_none() {
            let player = self.base.get_node_as::<Node3D>("/root/Game/Player");
            self.player = Some(player);
        }
    }

    fn process(&mut self, _delta: f64) {
        if let Some(player) = &self.player {
            self.base.look_at(player.get_position());
        }
    }
}
#[export_enemy_functions(damage)]
#[godot_api]
impl Owen {
    fn i_die(&mut self) {
        self.base.queue_free()
    }
}
