defmodule TaskApiWeb.GoalController do
  use TaskApiWeb, :controller
  alias TaskApi.Repo
  alias TaskApi.Goal

  def index(conn, _params) do
    goals = Repo.all(Goal)
    json(conn, Enum.map(goals, &goal_to_map/1))
  end

  def create(conn, %{"goal" => goal_params}) do
    changeset = Goal.changeset(%Goal{}, goal_params)

    case Repo.insert(changeset) do
      {:ok, goal} ->
        json(conn, %{data: goal_to_map(goal)})
      {:error, changeset} ->
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{errors: format_errors(changeset)})
    end
  end

  def show(conn, %{"id" => id}) do
    goal = Repo.get!(Goal, id)
    json(conn, %{data: goal_to_map(goal)})
  end

  def update(conn, %{"id" => id, "goal" => goal_params}) do
    goal = Repo.get!(Goal, id)
    changeset = Goal.changeset(goal, goal_params)

    case Repo.update(changeset) do
      {:ok, goal} ->
        json(conn, %{data: goal_to_map(goal)})
      {:error, changeset} ->
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{errors: format_errors(changeset)})
    end
  end

  def delete(conn, %{"id" => id}) do
    goal = Repo.get!(Goal, id)
    Repo.delete!(goal)
    send_resp(conn, :no_content, "")
  end

  defp goal_to_map(%Goal{} = goal) do
    %{
      id: goal.id,
      title: goal.title,
      description: goal.description,
      tasks_required: goal.tasks_required,
      tasks_completed: goal.tasks_completed
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
