use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
