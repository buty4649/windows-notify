mod toast;

use toast::*;

use seahorse::{error::FlagError, ActionError, ActionResult, App, Context, Flag, FlagType};

use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} <text> [<text>]", env!("CARGO_PKG_NAME")))
        .flag(
            Flag::new("app-id", FlagType::String)
                .description("The App ID to use for the notification")
                .alias("a"),
        )
        .flag(
            Flag::new("duration", FlagType::String)
                .description("The duration to show the notification. choose from 'short' or 'long'")
                .alias("d"),
        )
        .action_with_result(do_action);

    match app.run_with_result(args) {
        Ok(_) => {}
        Err(ActionError { message }) => {
            eprintln!("Error: {}", message);
            exit(1)
        }
    }
}

fn do_action(c: &Context) -> ActionResult {
    if c.args.is_empty() {
        return Err(ActionError {
            message: "No arguments provided".to_string(),
        });
    }

    macro_rules! flag {
        ($name:expr) => {
            match c.string_flag($name) {
                Ok(value) => Some(value),
                Err(e) => match e {
                    FlagError::NotFound => None,
                    _ => {
                        return Err(ActionError {
                            message: format!("{e}"),
                        })
                    }
                },
            }
        };
    }

    let text1 = &c.args[0];
    let app_id = flag!("app-id").unwrap_or("wnotify".to_string());
    let mut toast = toast::Toast::new(&app_id, text1);

    if let Some(text2) = c.args.get(1) {
        toast.text2(text2);
    }

    if let Some(duration) = flag!("duration") {
        toast.duration(match duration.as_str() {
            "short" => Duration::Short,
            "long" => Duration::Long,
            _ => {
                return Err(ActionError {
                    message: format!("Invalid duration: {}", duration),
                })
            }
        })
    }

    toast.notify();

    Ok(())
}
