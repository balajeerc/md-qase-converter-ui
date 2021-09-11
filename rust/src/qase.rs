use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub position: i32,
    pub action: String,
    pub expected_result: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Case {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub preconditions: Option<String>,
    pub postconditions: Option<String>,
    pub priority: String,
    pub severity: String,
    pub behavior: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub test_type: String,
    pub layer: String,
    pub is_flaky: String,
    pub automation: String,
    pub status: String,
    pub milestone: Option<String>,
    pub custom_fields: Vec<String>,
    pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suite {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub preconditions: Option<String>,
    pub cases: Vec<Case>,
    pub suites: Vec<Suite>,
}
