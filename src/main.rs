mod ai_functions;
mod agents;
mod models;
mod utils;

use utils::command_line::get_user_input;

fn main() {
    println!(
        "Welcome to Autumn!\n
        =====================================
    "
    );

    let prompt_project: String = get_user_input("What website are we building?", 1);
    let prompt_language: String =
        get_user_input("What language are we using? [Rust], [Python], [Go]", 2);
    let prompt_fullstack: String =
        get_user_input("Are we building [backend], [frontend], or [fullstack]?", 3);
    let _ = get_user_input("Exit", 4);

    println!(
        "{} {} {}",
        prompt_project, prompt_language, prompt_fullstack
    );
}
