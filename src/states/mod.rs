pub mod player_card_state;
pub mod card_game_state;

pub mod prelude {
    pub use crate::states::card_game_state::*;
    pub use crate::states::player_card_state::*;
}