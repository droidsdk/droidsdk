use seahorse::Command;
use crate::cli::activity::begin::build_cli_begin;

mod begin;

pub fn build_cli_activity() -> Command {
    // FIXME
    return build_cli_begin();
}