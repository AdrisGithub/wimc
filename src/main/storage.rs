use aul::error;
use aul::level::Level;
use aul::log;
use std::collections::HashMap;
use wimcm::WIMCError;
use wjp::{ParseError, Serialize, Values};

use crate::models::WIMCData;
use crate::saver::load;
use crate::util::is_due;

#[derive(Debug, Clone)]
pub struct Storage(HashMap<u128, WIMCData>);
impl Drop for Storage {
    fn drop(&mut self) {
        self.save();
        println!("Dropping");
    }
}
impl Storage {
    fn save(&self) {
        let _ = crate::saver::save(self.json().as_str()).map_err(|err| error!("{:?}", err));
    }
    pub fn store(&mut self, data: WIMCData) -> u128 {
        let id = *data.id();
        self.0.insert(id, data);
        self.save();
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
            self.0.remove(&value);
        }
        self.save();
    }
    fn _due_vals(&self) -> Vec<u128> {
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
        self.0.remove(&key);
        self.save();
    }
    pub fn new() -> Self {
        Self::load().unwrap_or(Storage(HashMap::new()))
    }
    fn load() -> Result<Storage, WIMCError> {
        Self::deserialize_str(&load()?).map_err(|_err| WIMCError)
    }
}
use wjp::Deserialize;
impl TryFrom<Values> for Storage {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let map = HashMap::try_from(value)?;
        Ok(Self(map))
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
    use wbdl::Date;
    use wjp::{Deserialize, Serialize};

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

    #[test]
    pub fn persistence() {
        let mut storage = Storage::new();
        let mut date = Date::now_unchecked();
        date.add_year();
        storage.store(WIMCData::default().with_id(10).with_time(date));
        drop(storage);
        let mut storage = Storage::new();
        println!("{:?}", storage.get(&10));
    }
    #[test]
    pub fn idk() {
        println!("{}", Storage::new().json());
    }
    #[test]
    pub fn idkk() {
        let mut storage = Storage::new();
        storage.store(WIMCData::default());
        storage.store(WIMCData::default().with_id(2));
        let json = storage.json();
        println!("{}", json);
        println!("{:?}", Storage::deserialize(json));
    }
}
