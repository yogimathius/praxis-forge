defmodule TaskApiWeb.Resolvers.Goals do
  require Logger
  alias TaskApi.Repo
  alias TaskApi.Goal

  def list_goals(_parent, _args, _resolution) do
    Logger.info("Fetching all goals")
    {:ok, Repo.all(Goal)}
  end

  def create_goal(_parent, args, _resolution) do
    %Goal{}
    |> Goal.changeset(args)
        |> Repo.insert()
    |> case do
      {:ok, goal} ->
        Logger.info("Successfully created goal: #{inspect(goal)}")
        {:ok, goal}
      {:error, changeset} ->
        Logger.error("Failed to create goal: #{inspect(changeset.errors)}")
        {:error, changeset}
    end
  end

  def update_goal(_parent, args, _resolution) do
    %Goal{}
    |> Goal.changeset(args)
    |> Repo.update( )
    |> case do
      {:ok, goal} ->
        Logger.info("Successfully updated goal: #{inspect(goal)}")
        {:ok, goal}
      {:error, changeset} ->
        Logger.error("Failed to update goal: #{inspect(changeset.errors)}")
        {:error, changeset}
    end
  end

  def delete_goal(_parent, args, _resolution) do
    %Goal{}
    |> Goal.changeset(args)
    |> Repo.delete()
    |> case do
      {:ok, goal} ->
        Logger.info("Successfully deleted goal: #{inspect(goal)}")
        {:ok, goal}
      {:error, changeset} ->
        Logger.error("Failed to delete goal: #{inspect(changeset.errors)}")
        {:error, changeset}
    end
  end
end
