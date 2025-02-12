defmodule TaskApiWeb.Schema do
  use Absinthe.Schema

  import_types(TaskApiWeb.Schema.Types)

  query do
    @desc "Get all tasks"
    field :tasks, list_of(:task) do
      resolve(&TaskApiWeb.Resolvers.Tasks.list_tasks/3)
    end

    @desc "Get all goals"
    field :goals, list_of(:goal) do
      resolve(&TaskApiWeb.Resolvers.Goals.list_goals/3)
    end
  end

  mutation do
    @desc "Create a task"
    field :create_task, :task do
      arg(:title, non_null(:string))
      arg(:description, :string)
      arg(:goal_id, :id)

      resolve(&TaskApiWeb.Resolvers.Tasks.create_task/3)
    end

    @desc "Create a goal"
    field :create_goal, :goal do
      arg(:title, non_null(:string))
      arg(:description, :string)
      arg(:tasks_required, non_null(:integer))

      resolve(&TaskApiWeb.Resolvers.Goals.create_goal/3)
    end
  end
end
