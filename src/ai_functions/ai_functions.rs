use ai_func_proc_macro::ai_function_to_string;
// use ai_functions::ai_function;

/* Architect AI Functions*/
#[ai_function_to_string]
pub fn print_project_scope(_project_description: &str) {
    /// Input: Takes in a user request to build a website project description
    /// Function: Converts user request into JSON response of information items required for a website build.
    /// Important: At least one of the bool results must be true
    /// Output: Prints an object response in the following format:
    ///   {
    ///     "is_crud_required": bool, // true if site needs CRUD functionality
    ///     "is_user_login_and_logout": bool // true if site needs users to be able to log in and log out
    ///     "is_external_urls_required": bool // true if site needs to fetch data from third part providers
    ///   }
    /// Example 1:
    ///   user_request = "I need a full stack website that accepts users and gets stock price data"
    ///   prints:
    ///   {
    ///     "is_crud_required": true
    ///     "is_user_login_and_logout": true
    ///     "is_external_urls_required": bool true
    ///   }
    /// Example 2:
    ///   user_request = "I need a simple TODO app"
    ///   prints:
    ///   {
    ///     "is_crud_required": true
    ///     "is_user_login_and_logout": false
    ///     "is_external_urls_required": bool false
    ///   }
    println!(OUTPUT)
}

#[ai_function_to_string]
pub fn print_site_urls(_project_description: &str) {
    /// Input: Takes in a project description of a website build
    /// Function: Outputs a list of external public API endpoints that should be used in the building of the website
    /// Important: Only selects url endpoint(s) which do not require any API Keys at all
    /// Output: Prints a list response of external urls in the following format:
    /// ["url1", "url2", "url3", ...]
    /// Example 1:
    ///   website_team_spec = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)"
    ///   prints:
    /// ["https://api.binance.com/api/v3/exchangeInfo", "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d"]
    println!(OUTPUT)
}

/* Project Manager AI Function */
#[ai_function_to_string]
pub fn convert_user_input_to_goal(_usr_req: &str) {
    /// Input: Takes in a user request
    /// Function: Converts user request into a short summarized goal
    /// Output: Prints goal. All outputs start with "build a website that ..."
    /// Example 1:
    ///   user_request = "I need a website that lets users login and logout. It needs to look fancy and accept payments."
    ///   OUTPUT = "build a website that handles users logging in and logging out and accepts payments"
    /// Example 2:
    ///   user_request = "Create something that stores crypto price data in a database using supabase and retrieves prices on the frontend."
    ///   OUTPUT = "build a website that fetches and stores crypto price data within a supabase setup including a frontend UI to fetch the data."
    println!(OUTPUT)
}

/* Backend Developer AI Functions */



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests_ai_function_proc_macros() {
        let output1 = print_project_scope("build a project");

        dbg!(output1);
    }
}
