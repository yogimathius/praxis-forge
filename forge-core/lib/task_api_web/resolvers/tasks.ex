defmodule TaskApiWeb.Resolvers.Tasks do
  require Logger
  alias TaskApi.Repo
  alias TaskApi.Task
  import Ecto.Query

  def list_tasks(_parent, _args, _resolution) do
    Logger.info("Fetching all tasks with associated goals")

    tasks = Task
    |> Repo.all()
    |> Repo.preload(:goal)

    {:ok, tasks}
  end

  def create_task(_parent, args, _resolution) do
    Logger.info("Creating task with args: #{inspect(args)}")
    result = %Task{}
    |> Task.changeset(args)
    |> Repo.insert()

    case result do
      {:ok, task} ->
        Logger.info("Successfully created task: #{inspect(task)}")
        # Preload the goal after creation
        {:ok, Repo.preload(task, :goal)}
      {:error, changeset} ->
        Logger.error("Failed to create task: #{inspect(changeset.errors)}")
        result
    end
  end

  def update_task(_parent, %{id: id} = args, _resolution) do
    Logger.debug("→ UPDATE TASK STARTED")
    Logger.debug("→ Received args: #{inspect(args, pretty: true)}")

    case Repo.get(Task, id) do
        nil ->
            Logger.warn("→ Task not found with ID: #{id}")
            {:error, "Task not found"}
        task ->
            Logger.debug("→ Found existing task: #{inspect(task, pretty: true)}")

            changeset = Task.changeset(task, args)
            Logger.debug("→ Built changeset: #{inspect(changeset, pretty: true)}")

            case Repo.update(changeset) do
                {:ok, updated_task} ->
                    Logger.debug("→ Successfully updated task: #{inspect(updated_task, pretty: true)}")
                    # Preload the goal after update
                    {:ok, Repo.preload(updated_task, :goal)}
                {:error, failed_changeset} ->
                    Logger.error("→ Update failed with errors: #{inspect(failed_changeset.errors, pretty: true)}")
                    {:error, "Failed to update task"}
            end
    end
  end

  def delete_task(_parent, %{id: id}, _resolution) do
    Logger.info("Attempting to delete task with id: #{id}")

    case Repo.get(Task, id) do
      nil ->
        {:error, "Task not found"}

      task ->
        # Preload the goal before deletion to include it in the response
        task = Repo.preload(task, :goal)

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
