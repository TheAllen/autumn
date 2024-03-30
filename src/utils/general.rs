use reqwest::Client;
use std::fs;

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let res: reqwest::Response = client.get(url).send().await?;
    Ok(res.status().as_u16())
}

pub fn save_code_to_file(filepath: &str, content: &str) {
    fs::write(filepath, content)
        .expect(format!("Could not write to filepath: {}", filepath).as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_code_file() {
        save_code_to_file("./generated_code/code_file.txt", "Testing Testing");
    }
}
