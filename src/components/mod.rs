pub mod components;
pub mod player;
pub mod npc;
pub mod cards;
pub mod collider;

pub mod prelude {
    pub use crate::components::components::*;
    pub use crate::components::player::*;
    pub use crate::components::npc::*;
    pub use crate::components::collider::*;
    pub use crate::components::cards::card::*;
    pub use crate::components::cards::water_card::*;
    pub use crate::components::cards::fire_card::*;
}