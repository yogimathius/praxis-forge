defmodule TaskApiWeb.Resolvers.Goals do
  alias TaskApi.Repo
  alias TaskApi.Goal

  def list_goals(_parent, _args, _resolution) do
    {:ok, Repo.all(Goal)}
  end

  def create_goal(_parent, args, _resolution) do
    %Goal{}
    |> Goal.changeset(args)
    |> Repo.insert()
  end
end
