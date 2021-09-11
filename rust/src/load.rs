use std::fs;
use std::path::Path;

// use crate::errors::ParseError;
use crate::qase::Suite;

pub fn load_test_cases_from_json(json_file: &Path) -> Suite {
    let json_text =
        fs::read_to_string(json_file.to_str().unwrap()).expect("file error loading json");
    let markdown_str = &json_text.to_string();

    let result = serde_json::from_str(&markdown_str).unwrap();
    return result;
}

// #[cfg(test)]
// mod tests {
//     use crate::markdown::load_calendar_mentions;
//     use std::path::Path;
//
//     #[test]
//     fn test_loading_dates_from_markdown() {
//         load_calendar_mentions(Path::new("./tests"));
//         assert_eq!(2 + 2, 4);
//     }
// }
