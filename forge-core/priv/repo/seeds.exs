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
    tasks_completed: 0
  },
  %{
    title: "Build Phoenix API",
    description: "Create a RESTful API using Phoenix framework.",
    tasks_required: 2,
    tasks_completed: 0
  },
  %{
    title: "Master Rust",
    description: "Complete Rust programming challenges.",
    tasks_required: 3,
    tasks_completed: 0
  }
]

Enum.each(goals, fn goal_data ->
  goal = Goal.changeset(%Goal{}, goal_data)
  |> Repo.insert!()

  # Create tasks for each goal
  tasks = [
    %{
      title: "Task 1 for #{goal_data.title}",
      status: "pending",
      description: "First task for #{goal_data.title}",
      completed: false,
      goal_id: goal.id  # Associate task with the goal
    },
    %{
      title: "Task 2 for #{goal_data.title}",
      status: "pending",
      description: "Second task for #{goal_data.title}",
      completed: false,
      goal_id: goal.id  # Associate task with the goal
    },
    %{
      title: "Task 3 for #{goal_data.title}",
      status: "pending",
      description: "Third task for #{goal_data.title}",
      completed: false,
      goal_id: goal.id  # Associate task with the goal
    }
  ]

  # Insert all tasks for the current goal
  Enum.each(tasks, fn task_data ->
    Task.changeset(%Task{}, task_data)
    |> Repo.insert!()
  end)
end)

IO.puts "Seeds planted! 🌱"
