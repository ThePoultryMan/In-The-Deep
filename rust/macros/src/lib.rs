#[macro_export]
macro_rules! enemy_node_from_class {
    ($enemy:expr,$node:ty) => {
        $enemy.get_node_as::<$node>(".")
    };
}
