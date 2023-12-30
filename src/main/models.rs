use wbdl::Date;
use wjp::{map, ParseError, Serialize, SerializeHelper, Values};

#[derive(Debug,Clone)]
pub struct WIMCData {
    payload: Values,
    params: Vec<String>,
    id: u128,
    time: Date,
}

impl Serialize for WIMCData {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("id", &self.id),
            ("payload", &self.payload),
            ("time", &self.time.to_string()),
            ("params", &self.params)
        ))
    }
}
impl TryFrom<Values> for WIMCData {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        //println!("{:?}", struc);
        let id = struc.map_val("id", u128::try_from)?;
        let params = struc.map_val("params", Vec::try_from)?;
        let payload = struc.remove("payload").ok_or(ParseError::new())?;
        let time = Date::try_from(
            struc
                .remove("time")
                .ok_or(ParseError::new())?
                .get_string()
                .ok_or(ParseError::new())?,
        )
        .map_err(|_err| ParseError::new())?;
        Ok(Self {
            id,
            params,
            time,
            payload,
        })
    }
}

impl Default for WIMCData {
    fn default() -> Self {
        Self {
            id: 0,
            params: Vec::default(),
            payload: Values::Null,
            time: Date::UNIX_EPOCH,
        }
    }
}

impl WIMCData {
    pub fn with_payload(mut self, payload: Values) -> Self {
        self.payload = payload;
        self
    }
    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }
    pub fn with_time(mut self, time: Date) -> Self {
        self.time = time;
        self
    }
    pub fn with_id(mut self, id: u128) -> Self {
        self.id = id;
        self
    }
    pub fn payload(&self) -> &Values {
        &self.payload
    }
    pub fn time(&self) -> &Date {
        &self.time
    }
    pub fn id(&self) -> &u128 {
        &self.id
    }
    pub fn params(&self) -> &Vec<String> {
        &self.params
    }
}
