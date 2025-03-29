use std::process::Command;

pub fn get_staged_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()?;

    Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect())
}

pub fn is_large_or_binary(file: &str) -> bool {
    [".pdf", ".png", ".jpg", ".jpeg", ".zip", ".mp4", ".exe"]
        .iter()
        .any(|ext| file.ends_with(ext))
}

pub fn get_filtered_diff(files: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    if files.is_empty() {
        return Ok(String::new());
    }

    let mut args = vec!["diff", "--cached"];
    args.extend(files.iter().map(String::as_str));

    let output = Command::new("git").args(args).output()?;
    Ok(String::from_utf8(output.stdout)?)
}
