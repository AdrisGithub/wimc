use wbdl::Date;
use wimcm::{WIMCError, WIMCInput, WIMCOutput};
use wjp::{Serialize, Values};

use crate::models::WIMCData;
use crate::storage::Storage;
use crate::util::find_date;

#[derive(Clone)]
pub struct Parser(Option<Storage>, u128);

impl Parser {
    pub const fn new() -> Self {
        Self(None, 0)
    }
    fn _store(&mut self) -> &mut Storage {
        if self.0.is_some() {
            self.0.as_mut().unwrap()
        } else {
            self.0 = Some(Storage::new());
            self.0.as_mut().unwrap()
        }
    }
    pub fn store(&mut self, input: WIMCInput) -> WIMCOutput {
        let i = self.increment();
        WIMCOutput::from_values(self._store().store(Self::map(input, i)).serialize())
    }
    fn increment(&mut self) -> u128 {
        let id = self.1;
        self.1 += 1;
        id
    }
    fn map(input: WIMCInput, id: u128) -> WIMCData {
        let mut vec = input.get_params().to_vec();
        WIMCData::default()
            .with_time(Self::_find_date(&mut vec))
            .with_params(input.get_params().to_vec())
            .with_payload(input.get_payload().serialize())
            .with_id(id)
    }
    fn _find_date(vec: &mut Vec<String>) -> Date {
        let opt = find_date(vec);
        if let Some(s) = opt {
            s
        } else {
            Date::now().unwrap_or(Date::UNIX_EPOCH).add_month()
        }
    }
    pub fn get(&mut self, input: WIMCInput) -> WIMCOutput {
        let id = u128::try_from(input.get_payload().serialize());
        match id {
            Ok(ok) => self
                ._store()
                .get(&ok)
                .map(Self::_mapp)
                .unwrap_or_else(|| WIMCOutput::from(WIMCError)),
            Err(_err) => WIMCOutput::from(WIMCError),
        }
    }
    fn _mapp(data: &WIMCData) -> WIMCOutput {
        WIMCOutput::from_values(data.payload().serialize())
    }
    pub fn query(&mut self, input: WIMCInput) -> WIMCOutput {
        let idk: Vec<Values> = self
            ._store()
            .query(input.get_params().to_vec())
            .into_iter()
            .map(|v| v.payload().serialize())
            .collect();
        WIMCOutput::from_values(idk.serialize())
    }
    pub fn remove(&mut self, input: WIMCInput) -> WIMCOutput {
        let id = u128::try_from(input.get_payload().serialize());
        if let Ok(id) = id {
            self._store().remove(id)
        }
        WIMCOutput::from_values(Values::Null)
    }
    pub fn cleanup(&mut self) -> WIMCOutput {
        self._store().cleanup();
        WIMCOutput::from_values(Values::Null)
    }
}
