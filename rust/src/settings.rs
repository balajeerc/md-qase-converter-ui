use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub markdown_notes_directory: String,
    pub calendar_tag_string: String,
}

impl ::std::default::Default for Settings {
    fn default() -> Self {
        Self {
            markdown_notes_directory: "".into(),
            calendar_tag_string: "Calendar".into(),
        }
    }
}

pub fn load_settings() -> Result<Settings, confy::ConfyError> {
    return confy::load("lifemd");
}
