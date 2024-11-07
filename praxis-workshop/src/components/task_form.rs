use yew::{function_component, html, use_node_ref, Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub on_add: Callback<String>,
}

#[function_component(TaskForm)]
pub fn task_form(props: &TaskFormProps) -> Html {
    let input_ref = use_node_ref();

    let onsubmit = {
        let input_ref = input_ref.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            on_add.emit(input.value());
            input.set_value("");
        })
    };

    html! {
        <form onsubmit={onsubmit}>
            <input ref={input_ref} placeholder="New task" />
            <button type="submit">{"Add Task"}</button>
        </form>
    }
}
