use std::collections::HashMap;

use aul::error;
use aul::level::Level;
use aul::log;
use wjp::{ParseError, Serialize, Values};

use crate::models::WIMCData;

struct Storage {
    store: HashMap<u128, WIMCData>,
}
impl Drop for Storage {
    fn drop(&mut self) {
        let _ = crate::saver::save(self.json().as_str()).map_err(|err| error!("{:?}", err));
    }
}
impl Storage {
    pub fn store(&mut self, data: WIMCData) -> u128 {
        let id = *data.id();
        self.store.insert(id, data);
        id
    }
    pub fn get(&mut self, id: &u128) -> Option<&WIMCData> {
        self.store.get(id)
    }
    pub fn query(&mut self, words: Vec<String>) -> Vec<&WIMCData> {
        self.store
            .values()
            .filter(|&val| words.iter().all(|word| val.params().contains(word))) // TODO wichtig abtesten
            .collect()
    }
}

impl TryFrom<Values> for Storage {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        Ok(Self {
            store: HashMap::try_from(value)?,
        })
    }
}
impl Serialize for Storage {
    fn serialize(&self) -> Values {
        self.store.serialize()
    }
}
