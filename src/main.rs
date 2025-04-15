use fake::Fake;
use fake::faker::internet::raw::SafeEmail;
use fake::locales::EN;
use std::io::Read;

fn main() {
    // Read input value from stdin
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // Remove curly braces and split by comma
    let trimmed: &str = input.trim().trim_start_matches('{').trim_end_matches('}');
    let values: Vec<String> = trimmed.split(',').map(String::from).collect();

    // Transform each value with the original RepliByte EmailTransformer logic
    let transformed_values: Vec<String> = values
        .iter()
        .map(|s| match s.len() {
            len if len == 0 => s.clone(),
            _ => SafeEmail(EN).fake(),
        })
        .collect();

    // Write transformed value to stdout (simply print)
    println!("{{{}}}", transformed_values.join(","));
}
