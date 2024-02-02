use crate::components::cards::card::*;
use crate::states::card_game_state::*;

struct FireCard(bool);

impl Card for FireCard {
    fn take_effect(&self, cg_state: &mut CardGameState) {
        cg_state.adjust("Red", 1);
    }
}