alias TaskApi.Repo
alias TaskApi.Task

# Clear existing data
Repo.delete_all(Task)

# Insert mock data
Repo.insert!(%Task{title: "Learn Elixir", completed: false})
Repo.insert!(%Task{title: "Build Phoenix API", completed: false})
Repo.insert!(%Task{title: "Integrate with Tauri", completed: false})
