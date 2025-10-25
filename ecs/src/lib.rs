mod components;
pub use components::Components;
mod world;
pub use world::World;

use uuid::Uuid;
pub type Entity = Uuid;
