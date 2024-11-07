use stylist::{style, yew::styled_component};
use yew::{html, use_node_ref, Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub on_add: Callback<String>,
}

#[styled_component]
pub fn TaskForm(props: &TaskFormProps) -> Html {
    let input_ref = use_node_ref();

    let form_styles = style!(
        r#"
        margin-bottom: 1.5rem;
    "#
    )
    .unwrap();

    let container_styles = style!(
        r#"
        display: flex;
        gap: 1rem;
    "#
    )
    .unwrap();

    let input_styles = style!(
        r#"
        flex: 1;
        padding: 0.5rem 1rem;
        border: 1px solid #e2e8f0;
        border-radius: 0.25rem;
        &:focus {
            outline: none;
            box-shadow: 0 0 0 2px #3b82f6;
        }
    "#
    )
    .unwrap();

    let button_styles = style!(
        r#"
        background-color: #3b82f6;
        color: white;
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
        border: none;
        cursor: pointer;
        &:hover {
            background-color: #2563eb;
        }
    "#
    )
    .unwrap();

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
        <form class={form_styles} onsubmit={onsubmit}>
            <div class={container_styles}>
                <input
                    type="text"
                    class={input_styles}
                    placeholder="Add a new task..."
                    ref={input_ref}
                />
                <button
                    type="submit"
                    class={button_styles}
                >
                    {"Add Task"}
                </button>
            </div>
        </form>
    }
}
