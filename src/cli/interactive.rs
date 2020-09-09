use seahorse::{App, Command, Context, Flag, FlagType, error::FlagError};
use crate::cli::root::build_cli_root;
use dialoguer::Input;
use std::error::Error;

pub fn build_cli_interactive() -> Command {
    Command::new("interactive")
        .usage("cli interactive")
        .action(exec_interactive)
}

pub fn exec_interactive(c: &Context) {
    loop {
        // TODO prevent nesting of interactive shells
        //  maybe delegate to bash?

        let exec = || -> Result<bool, Box<dyn Error>> {
            let args_unparsed: String = Input::new()
                .with_prompt("interactive")
                .interact()?;
            // TODO better arg splitting
            let args = &mut args_unparsed.split(" ").map(|x| x.to_owned()).collect::<Vec<String>>();

            if args[0] == "exit" {
                return Ok(false);
            }

            let call = &mut vec!["cmd".to_owned()];
            call.append(args);

            let app = build_cli_root();
            app.run(call.to_vec());
            return Ok(true);
        };

        let result = exec();
        if result.is_err() {
            eprintln!("Encountered an error during execution!");
            eprintln!("{}", result.unwrap_err());
            break;
        }

        if !result.unwrap() {
            println!("Exiting.");
            break;
        }
    }
}