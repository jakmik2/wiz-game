use crate::states::card_game_state::*;

pub trait Card {
    fn take_effect(&self, cg_state: &mut CardGameState);
}