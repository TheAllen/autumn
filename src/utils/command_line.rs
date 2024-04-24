use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintMessage {
    Info,
    Testing,
    Error
}

impl PrintMessage {
    pub fn print_agent_msg(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();
        let msg_color: Color = match self {
            Self::Info => Color::Rgb { // blue color
                r: 61,
                g : 103,
                b: 242
            },
            Self::Testing => Color::Rgb { // Yellow color
                r: 219,
                g : 206,
                b: 88
            },
            Self::Error => Color::Rgb { // Red color
                r: 219,
                g : 88,
                b: 129
            }
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {} - ", agent_pos);

        stdout.execute(SetForegroundColor(msg_color)).unwrap();
        println!("{}", agent_statement);

        // Reset text color back
        stdout.execute(ResetColor).unwrap();
    }
}

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

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!();
        print!("WARNING: The code is AI generated. ");
        println!("Review the code before proceeding.");
        println!();

        // Reset color
        stdout.execute(ResetColor).unwrap();

        // Present options
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1][yes][y] Looks good!");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2][no][n] Let's drop this project.");

        // Reset color
        stdout.execute(ResetColor).unwrap();

        let mut user_resp: String = String::new();
        stdin()
            .read_line(&mut user_resp)
            .expect("Failed to read user response");

        // Trim whitespace and convert to lowercase
        let user_resp: String = user_resp.trim().to_lowercase();

        match user_resp.as_str() {
            "1" | "yes" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!(r#"Invalid input. Please select "1" or "2""#);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests_command_line_messages() {
        
        // Info
        PrintMessage::Info.print_agent_msg("Project Manager", "Converting user input to goal");

        // Testing
        PrintMessage::Testing.print_agent_msg("Backend engineer", "Testing backend code");

        // Error
        PrintMessage::Error.print_agent_msg("Solutions Architect", "Testing Error");
    }

    #[test]
    fn tests_confirm_test_code() {
        println!("{}", confirm_safe_code());
    }
}