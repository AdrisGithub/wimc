use std::collections::HashMap;

use aul::error;
use aul::level::Level;
use aul::log;
use wimcm::WIMCError;
use wjp::{Deserialize, ParseError, Serialize, Values};

use crate::models::WIMCData;
use crate::saver::load;
use crate::util::is_due;
#[derive(Debug)]
pub struct Storage(HashMap<u128, WIMCData>);
impl Drop for Storage {
    fn drop(&mut self) {
        let _ = crate::saver::save(self.json().as_str()).map_err(|err| error!("{:?}", err));
        println!("Dropping");
    }
}
impl Storage {
    pub fn store(&mut self, data: WIMCData) -> u128 {
        let id = *data.id();
        self.0.insert(id, data);
        id
    }
    pub fn get(&mut self, id: &u128) -> Option<&WIMCData> {
        self.cleanup();
        self.0.get(id)
    }
    pub fn query(&mut self, words: Vec<String>) -> Vec<&WIMCData> {
        self.cleanup();
        self.0
            .values()
            .filter(|&val| words.iter().all(|word| val.params().contains(word)))
            .collect() // TODO wichtig abtesten
    }

    pub fn cleanup(&mut self) {
        for value in self._due_vals().iter() {
            self.remove(*value);
        }
    }
    pub fn _due_vals(&self) -> Vec<u128> {
        let mut new = Vec::with_capacity(self.0.len());
        for value in self._values() {
            if is_due(value.time()) {
                new.push(*value.id())
            }
        }
        new
    }
    fn _values(&self) -> Vec<&WIMCData> {
        self.0.values().collect()
    }
    pub fn remove(&mut self, key: u128) {
        println!("{:?}", self.0.remove(&key));
    }
    pub fn new() -> Self {
        Self::load().unwrap_or(Storage(HashMap::new()))
    }
    fn load() -> Result<Storage, WIMCError> {
        Self::deserialize(load()?).map_err(|err| WIMCError)
    }
}

impl TryFrom<Values> for Storage {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        Ok(Self(HashMap::try_from(value)?))
    }
}
impl Serialize for Storage {
    fn serialize(&self) -> Values {
        self.0.serialize()
    }
}
#[cfg(test)]
mod tests {
    use crate::models::WIMCData;
    use crate::storage::Storage;

    #[test]
    pub fn it_test() {
        let mut storage = Storage::new();
        let data = WIMCData::default();
        let id = *data.id();
        storage.store(data);
        storage.get(&id);
    }
    #[test]
    pub fn testing() {
        let mut storage = Storage::new();
        let data = WIMCData::default().with_id(3);
        storage.store(data);
        let data = WIMCData::default().with_id(2);
        storage.store(data);
        let data = WIMCData::default().with_id(1);
        storage.store(data);
    }
}
