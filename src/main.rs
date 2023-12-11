mod toast;

use seahorse::{ActionError, ActionResult, App, Context, Flag, FlagType};

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

    let app_id = if let Ok(app_id) = c.string_flag("app-id") {
        app_id
    } else {
        "wnotify".to_string()
    };

    let text1 = &c.args[0];
    let text2 = c.args.get(1).map(|s| s.as_str());

    let toast = toast::Toast::new(&app_id, text1, text2);
    toast.notify();

    Ok(())
}
