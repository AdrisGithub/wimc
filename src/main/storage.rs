use std::collections::HashMap;

use aul::error;
use aul::level::Level;
use aul::log;
use wjp::{ParseError, Serialize, Values};

use crate::models::WIMCData;

struct Storage {
    store: HashMap<u128,WIMCData>, 
}
impl Drop for Storage {
    fn drop(&mut self) {
        let _ = crate::saver::save(self.json().as_str()).map_err(|err| error!("{:?}", err));
    }
}
impl Storage {
    
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
