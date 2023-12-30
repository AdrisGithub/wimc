use crate::storage::Storage;

pub struct Parser(Option<Storage>);

impl Parser {
    pub const fn new() -> Self {
        Self(None)
    }
}
