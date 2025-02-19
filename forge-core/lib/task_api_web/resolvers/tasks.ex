defmodule TaskApiWeb.Resolvers.Tasks do
  require Logger
  alias TaskApi.Repo
  alias TaskApi.Task

  def list_tasks(_parent, _args, _resolution) do
    Logger.info("Fetching all tasks")
    {:ok, Repo.all(Task)}
  end

  def create_task(_parent, args, _resolution) do
    Logger.info("Creating task with args: #{inspect(args)}")
    result = %Task{}
    |> Task.changeset(args)
    |> Repo.insert()

    case result do
      {:ok, task} ->
        Logger.info("Successfully created task: #{inspect(task)}")
        result
      {:error, changeset} ->
        Logger.error("Failed to create task: #{inspect(changeset.errors)}")
        result
    end
  end

  def update_task(_parent, args, _resolution) do
    Logger.info("Updating task with args: #{inspect(args)}")
    result = %Task{}
    |> Task.changeset(args)
    |> Repo.update()

    case result do
      {:ok, task} ->
        Logger.info("Successfully updated task: #{inspect(task)}")
        result
      {:error, changeset} ->
        Logger.error("Failed to update task: #{inspect(changeset.errors)}")
        result
    end
  end

  def delete_task(_parent, %{id: id}, _resolution) do
    Logger.info("Attempting to delete task with id: #{id}")

    case Repo.get(Task, id) do
      nil ->
        {:error, "Task not found"}

      task ->
        case Repo.delete(task) do
          {:ok, deleted_task} ->
            Logger.info("Successfully deleted task with id: #{id}")
            {:ok, deleted_task}

          {:error, changeset} ->
            Logger.error("Failed to delete task: #{inspect(changeset.errors)}")
            {:error, "Failed to delete task"}
        end
    end
  end
end
