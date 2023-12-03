use std::error::Error;

pub fn read_data(day: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let filename = format!("input/{}.txt", day);
    let contents = std::fs::read_to_string(filename)?;
    let lines: Vec<String> = contents
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(lines)
}
