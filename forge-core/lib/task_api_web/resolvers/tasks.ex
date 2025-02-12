defmodule TaskApiWeb.Resolvers.Tasks do
  alias TaskApi.Repo
  alias TaskApi.Task

  def list_tasks(_parent, _args, _resolution) do
    {:ok, Repo.all(Task)}
  end

  def create_task(_parent, args, _resolution) do
    %Task{}
    |> Task.changeset(args)
    |> Repo.insert()
  end
end
