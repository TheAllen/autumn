use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

pub fn get_user_input(question: &str, question_num: u8) -> String {
    let mut stdout: std::io::Stdout = stdout();
    // Prompt
    match question_num {
        1 => stdout
            .execute(SetForegroundColor(Color::Rgb {
                r: 66,
                g: 135,
                b: 245,
            }))
            .unwrap(),
        2 => stdout
            .execute(SetForegroundColor(Color::Rgb {
                r: 55,
                g: 169,
                b: 203,
            }))
            .unwrap(),
        3 => stdout
            .execute(SetForegroundColor(Color::Rgb {
                r: 149,
                g: 219,
                b: 231,
            }))
            .unwrap(),
        _ => {
            stdout.execute(ResetColor).unwrap();
            return "".to_string();
        }
    };
    // Print prompt
    println!("{}", question);

    let mut user_input: String = String::new();
    stdin().read_line(&mut user_input).expect(
        format!(
            "Failed to read user input for question: {}",
            question.to_string()
        )
        .as_str(),
    );

    user_input
}
