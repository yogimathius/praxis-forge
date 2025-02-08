alias TaskApi.Repo
alias TaskApi.Task
alias TaskApi.Goal

# Clear existing data
Repo.delete_all(Task)
Repo.delete_all(Goal)

# Create list of goals
goals = [
  %{
    title: "Learn Elixir",
    description: "Complete Elixir tutorials and projects.",
    tasks_required: 3,
    tasks_completed: 0,
  },
  %{
    title: "Build Phoenix API",
    description: "Create a RESTful API using Phoenix framework.",
    tasks_required: 2,
    tasks_completed: 0,
  },
  %{
    title: "Master Rust",
    description: "Complete Rust programming challenges.",
    tasks_required: 3,
    tasks_completed: 0,
  }
]

# Insert goals and their tasks
Enum.each(goals, fn goal_data ->
  {:ok, goal} = Goal.changeset(%Goal{}, goal_data) |> Repo.insert()
  IO.puts("Created goal: #{inspect(goal)}")

  # Create tasks for each goal
  tasks = [
    %{
      title: "Task 1 for #{goal.title}",
      description: "First task for #{goal.title}",
      status: "pending",
      completed: false,
      goal_id: goal.id
    },
    %{
      title: "Task 2 for #{goal.title}",
      description: "Second task for #{goal.title}",
      status: "pending",
      completed: false,
      goal_id: goal.id
    },
    %{
      title: "Task 3 for #{goal.title}",
      description: "Third task for #{goal.title}",
      status: "pending",
      completed: false,
      goal_id: goal.id
    }
  ]

  Enum.each(tasks, fn task_data ->
    {:ok, task} = Task.changeset(%Task{}, task_data) |> Repo.insert()
    IO.puts("Created task: #{inspect(task)}")
  end)
end)

# Verify associations
tasks_with_goals = Task |> Repo.all() |> Repo.preload(:goal)
IO.puts("\nVerifying associations:")
Enum.each(tasks_with_goals, fn task ->
  IO.puts("Task '#{task.title}' belongs to goal '#{task.goal.title}'")
end)

IO.puts "\nSeeds planted! 🌱"
