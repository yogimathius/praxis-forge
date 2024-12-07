alias TaskApi.Repo
alias TaskApi.Task

# Clear existing data
Repo.delete_all(Task)

# Create list of tasks
tasks = [
  %{
    title: "Learn Elixir",
    status: "in_progress",
    completed: false
  },
  %{
    title: "Build Phoenix API",
    status: "completed",
    completed: true
  },
  %{
    title: "Master Rust",
    status: "pending",
    completed: false
  }
]

# Insert all tasks
Enum.each(tasks, fn task_data ->
  Task.changeset(%Task{}, task_data)
  |> Repo.insert!()
end)

IO.puts "Seeds planted! 🌱"
