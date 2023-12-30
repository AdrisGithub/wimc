use crate::storage::Storage;

pub struct Parser(Storage);

impl Parser {
    pub fn new() -> Self {
        Self(Storage::new())
    }
}
