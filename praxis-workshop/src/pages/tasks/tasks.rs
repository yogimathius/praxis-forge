use crate::components::task::form::TaskForm;
use crate::components::task::list::TasksList;
use crate::state::use_goals::{use_goals, GoalsState};
use crate::state::use_tasks::{use_tasks, TasksState};
use leptos::prelude::*;

#[component]
pub fn TasksListPage() -> impl IntoView {
    let TasksState {
        tasks,
        loading: _,
        error: _,
        create,
        update,
        delete,
        refetch,
    } = use_tasks();
    let GoalsState {
        goals,
        loading: _,
        error: _,
        create: _,
        update: _,
        delete: _,
        refetch: _,
    } = use_goals();

    view! {
        <div class="max-w-7xl mx-auto p-8 my-8">
            <h2 class="text-4xl font-bold text-spark text-center mb-8 animate-ember-pulse">
                "Forge Operations"
            </h2>
            <p class="text-xl text-center text-ash mb-12 animate-slide-up">
                "Shape your tasks into achievements, one strike at a time."
            </p>

            <div class="bg-glass-dark rounded-xl border border-orange-30 p-8 mb-12 animate-fade-in delay-300">
                <TaskForm
                    create=create.clone()
                    refetch=refetch.clone()
                    goals=goals
                />
            </div>

            <div class="bg-glass rounded-xl border border-orange-30 p-8 animate-fade-in delay-500">
                {
                    let tasks = tasks.clone();
                    move || -> View<_> {
                        let tasks = tasks.get().clone();
                        let on_toggle = update.clone();
                        let on_delete = delete.clone();
                        let on_edit = update.clone();
                        let goals = goals.clone();

                        view! {
                            <TasksList
                                tasks=tasks
                                on_toggle=on_toggle
                                on_delete=on_delete
                                on_edit=on_edit
                                goals=goals
                            />
                        }.into_view()
                    }
                }
            </div>
        </div>
    }
}
