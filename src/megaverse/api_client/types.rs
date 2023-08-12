use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GoalResponse {
    pub goal: Vec<Vec<String>>,
}
