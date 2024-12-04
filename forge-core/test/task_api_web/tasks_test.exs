defmodule TaskApiWeb.TasksTest do
  use TaskApiWeb.ConnCase
  alias TaskApi.Task

  test "create_task/1 with valid data creates a task" do
    assert {:ok, %Task{} = task} = Task.create_task(%{title: "New Task", status: "pending"})
    assert task.title == "New Task"
  end
end
