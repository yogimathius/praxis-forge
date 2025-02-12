#[allow(dead_code)]
pub mod schema {
    pub struct RootQueryType;
    pub struct RootMutationType;

    pub struct Task {
        pub id: Option<cynic::Id>,
        pub title: Option<String>,
        pub description: Option<String>,
        pub completed: Option<bool>,
        pub status: Option<String>,
        pub goal: Option<Goal>,
    }

    pub struct Goal {
        pub id: Option<cynic::Id>,
        pub title: Option<String>,
        pub description: Option<String>,
        pub tasks_required: Option<i32>,
        pub tasks_completed: Option<i32>,
        pub tasks: Option<Vec<Task>>,
    }
}

pub use schema::*;
