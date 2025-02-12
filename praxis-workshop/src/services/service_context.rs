use super::DataService;
use leptos::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceContext(pub Arc<dyn DataService>);

pub fn use_service() -> ServiceContext {
    use_context::<ServiceContext>().expect("ServiceContext not provided")
}
