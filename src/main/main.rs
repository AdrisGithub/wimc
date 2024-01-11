use wbsl::error::WBSLError;
use wbsl::ser_servlet::SerializeServlet;
use wimcm::{ADDRESS, DOUBLE_COLON, PORT, WIMCInput, WIMCMethods, WIMCOutput};
use wimcm::presets::{pong, respond};

use crate::parser::Parser;

mod constants;
mod models;
mod parser;
mod saver;
mod storage;
mod util;

fn main() -> Result<(), WBSLError> {
    SerializeServlet::builder()
        .with_func(handle_requests)
        .bind(format!("{}{}{}", ADDRESS, DOUBLE_COLON, PORT))?
        .start();
    Ok(())
}

fn handle_requests(input: WIMCInput) -> WIMCOutput {
    match input.get_method() {
        WIMCMethods::Ping => ping(),
        WIMCMethods::Echo => echo(input),
        WIMCMethods::Cleanup => cleanup(),
        WIMCMethods::Store => store(input),
        WIMCMethods::Get => get(input),
        WIMCMethods::Query => query(input),
        WIMCMethods::Remove => remove(input),
        WIMCMethods::StoreInc => store_inc(input),
    }
}

fn store_inc(input: WIMCInput) -> WIMCOutput {
    unsafe { STORE.store_inc(input) }
}

static mut STORE: Parser = Parser::new();

fn ping() -> WIMCOutput {
    pong()
}

fn echo(input: WIMCInput) -> WIMCOutput {
    respond(input.get_params().to_vec())
}

fn cleanup() -> WIMCOutput {
    unsafe { STORE.cleanup() }
}

fn store(input: WIMCInput) -> WIMCOutput {
    unsafe { STORE.store(input) }
}

fn get(input: WIMCInput) -> WIMCOutput {
    unsafe { STORE.get(input) }
}

fn query(input: WIMCInput) -> WIMCOutput {
    unsafe { STORE.query(input) }
}

fn remove(input: WIMCInput) -> WIMCOutput {
    unsafe { STORE.remove(input) }
}
