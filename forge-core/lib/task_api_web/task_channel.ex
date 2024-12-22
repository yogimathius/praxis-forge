defmodule TaskApiWeb.TaskChannel do
  use TaskApiWeb, :channel
  require Logger

  def join("tasks:updates", _payload, socket) do
    Logger.info("Client joined tasks channel")
    {:ok, socket}
  end

  # Will be called from the TaskController
  def broadcast_update(task) do
    TaskApiWeb.Endpoint.broadcast!("tasks:updates", "task_updated", task_to_map(task))
  end

  defp task_to_map(task) do
    %{
      id: task.id,
      title: task.title,
      completed: task.completed,
      description: task.description,
      status: task.status
    }
  end
end
