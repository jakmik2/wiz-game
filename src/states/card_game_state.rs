use std::collections::HashMap;

pub struct CardGameState {
    asset_counter: HashMap<String, i16>
}

impl CardGameState {
    pub fn new() -> Self {
        let asset_counter = HashMap::from([
            ("Red".to_string(), 0),
            ("Green".to_string(), 0),
            ("Blue".to_string(), 0),
        ]);

        Self {
            asset_counter
        }
    }

    pub fn reset(&mut self) {
        let reset_asset_counter = HashMap::from([
            ("Red".to_string(), 0),
            ("Green".to_string(), 0),
            ("Blue".to_string(), 0),
        ]);
        self.asset_counter = reset_asset_counter;
    }

    pub fn adjust(&mut self, asset_key: &str, amt: i16) {
        let asset = self.asset_counter.get_mut(asset_key).unwrap();
        *asset += amt;
    }
}