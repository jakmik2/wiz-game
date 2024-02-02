use crate::components::cards::card::*;
use crate::states::card_game_state::*;

struct WaterCard(bool);

impl Card for WaterCard {
    fn take_effect(&self, cg_state: &mut CardGameState) {
        cg_state.adjust("Blue", 2);
        cg_state.adjust("Red", -1);
    }
}