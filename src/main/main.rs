use wbsl::error::WBSLError;
use wbsl::ser_servlet::SerializeServlet;
use wimcm::presets::{pong, respond};
use wimcm::{WIMCInput, WIMCMethods, WIMCOutput};
use wjp::Values;

use crate::constants::{ADDRESS, DOUBLE_COLON, PORT};
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
    }
}
static mut STORE: Parser = Parser::new();

fn ping() -> WIMCOutput {
    pong()
}

fn echo(input: WIMCInput) -> WIMCOutput {
    respond(input.get_params().to_vec())
}

fn cleanup() -> WIMCOutput {
    WIMCOutput::from_values(Values::Null)
}

fn store(input: WIMCInput) -> WIMCOutput {
    todo!()
}

fn get(input: WIMCInput) -> WIMCOutput {
    todo!()
}

fn query(input: WIMCInput) -> WIMCOutput {
    todo!()
}
