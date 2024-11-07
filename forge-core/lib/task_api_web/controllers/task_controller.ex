defmodule TaskApiWeb.TaskController do
  use TaskApiWeb, :controller
  alias TaskApi.Repo
  alias TaskApi.Task
  require Logger

  def index(conn, _params) do
    Logger.info("Fetching all tasks")
    tasks = Repo.all(Task)
    Logger.info("Fetched #{length(tasks)} tasks")
    Logger.debug("Tasks: #{inspect(tasks)}")
    json(conn, %{data: Enum.map(tasks, &task_to_map/1)})
  end

  def create(conn, %{"task" => task_params}) do
    Logger.info("Creating a new task with params: #{inspect(task_params)}")
    changeset = Task.changeset(%Task{}, task_params)

    case Repo.insert(changeset) do
      {:ok, task} ->
        Logger.info("Task created successfully: #{inspect(task)}")
        conn
        |> put_status(:created)
        |> json(%{data: task_to_map(task)})
      {:error, changeset} ->
        Logger.warning("Failed to create task: #{inspect(changeset.errors)}")
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{errors: format_errors(changeset)})
    end
  end

  def update(conn, %{"id" => id, "task" => task_params}) do
    Logger.info("Updating task #{id} with params: #{inspect(task_params)}")
    task = Repo.get!(Task, id)
    changeset = Task.changeset(task, task_params)

    case Repo.update(changeset) do
      {:ok, task} ->
        Logger.info("Task #{id} updated successfully: #{inspect(task)}")
        json(conn, %{data: task_to_map(task)})
      {:error, changeset} ->
        Logger.warning("Failed to update task #{id}: #{inspect(changeset.errors)}")
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{errors: format_errors(changeset)})
    end
  end

  defp task_to_map(%Task{} = task) do
    %{
      id: task.id,
      title: task.title,
      completed: task.completed
    }
  end

  defp format_errors(changeset) do
    Ecto.Changeset.traverse_errors(changeset, fn {msg, opts} ->
      Enum.reduce(opts, msg, fn {key, value}, acc ->
        String.replace(acc, "%{#{key}}", to_string(value))
      end)
    end)
  end
end
