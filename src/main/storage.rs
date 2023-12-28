use aul::error;
use aul::level::Level;
use aul::log;
use wimcm::WIMCInput;
use wjp::{ParseError, Serialize, Values};

use crate::saver;

struct Storage {
    store: Vec<WIMCInput>, // TODO this is just an arbitary type for the start
}
impl Drop for Storage {
    fn drop(&mut self) {
        let _ = saver::save(self.json().as_str()).map_err(|err| error!("{:?}", err));
    }
}

impl TryFrom<Values> for Storage {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        Ok(Self {
            store: Vec::try_from(value)?,
        })
    }
}
impl Serialize for Storage {
    fn serialize(&self) -> Values {
        self.store.serialize()
    }
}
