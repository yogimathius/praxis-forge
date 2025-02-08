defmodule TaskApiWeb.TaskController do
  use TaskApiWeb, :controller
  require IEx
  import Ecto.Query
  alias TaskApi.Repo
  alias TaskApi.Task
  alias TaskApi.Goal
  require Logger

  def index(conn, _params) do
    Logger.info("Fetching all tasks")
    tasks = Task
      |> Repo.all()
      |> Repo.preload(:goal)
    Logger.info("Fetched #{length(tasks)} tasks")
    Logger.debug("Tasks: #{inspect(tasks)}")
    json(conn, Enum.map(tasks, &task_to_map/1))
  end

  def create(conn, %{"task" => task_params}) do
    Logger.info("Creating a new task with params: #{inspect(task_params)}")
    IEx.pry()
    changeset = Task.changeset(%Task{}, task_params)

    case Repo.insert(changeset) do
      {:ok, task} ->
        task = task |> Repo.preload(:goal) |> maybe_update_goal_progress()
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

  def update(conn, params) do
    Logger.debug("Raw update params: #{inspect(params)}")

    case params do
      %{"id" => id, "task" => task_params} ->
        Logger.info("Updating task #{id} with params: #{inspect(task_params)}")
        task = Task |> Repo.get!(id) |> Repo.preload(:goal)
        changeset = Task.changeset(task, task_params)

        Logger.debug("Changeset: #{inspect(changeset)}")

        case Repo.update(changeset) do
          {:ok, task} ->
            task = maybe_update_goal_progress(task)
            Logger.info("Task #{id} updated successfully: #{inspect(task)}")
            TaskApiWeb.TaskChannel.broadcast_update(task)
            json(conn, %{data: task_to_map(task)})
          {:error, changeset} ->
            Logger.warning("Failed to update task #{id}: #{inspect(changeset.errors)}")
            conn
            |> put_status(:unprocessable_entity)
            |> json(%{errors: format_errors(changeset)})
        end
      _ ->
        Logger.warning("Invalid update params received: #{inspect(params)}")
        conn
        |> put_status(:bad_request)
        |> json(%{error: "Invalid parameters"})
    end
  end

  def delete(conn, %{"id" => id}) do
    Logger.info("Deleting task #{id}")
    Repo.delete(Repo.get!(Task, id))
    send_resp(conn, :no_content, "")
  end

  defp task_to_map(%Task{} = task) do
    base_map = %{
      id: task.id,
      title: task.title,
      completed: task.completed,
      description: task.description,
      status: task.status,
      goal_id: task.goal_id
    }

    case task.goal do
      %Goal{} = goal ->
        Map.put(base_map, :goal_title, goal.title)
      _ ->
        base_map
    end
  end

  defp format_errors(changeset) do
    Ecto.Changeset.traverse_errors(changeset, fn {msg, opts} ->
      Enum.reduce(opts, msg, fn {key, value}, acc ->
        String.replace(acc, "%{#{key}}", to_string(value))
      end)
    end)
  end

  defp maybe_update_goal_progress(%Task{goal_id: goal_id} = task) do
    case goal_id do
      nil ->
        task
      _ ->
        goal = Repo.get!(Goal, goal_id)
        completed_tasks = Repo.one(
          from t in Task,
          where: t.goal_id == ^goal_id and t.completed == true,
          select: count(t.id)
        )

        if completed_tasks >= goal.tasks_required do
          Goal.changeset(goal, %{completed: true})
          |> Repo.update!()
        end

        Goal.changeset(goal, %{tasks_completed: completed_tasks})
        |> Repo.update!()

        Goal.changeset(goal, %{tasks_required: goal.tasks_required + 1})
        |> Repo.update!()

        task
    end
  end
end
