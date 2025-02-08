defmodule TaskApi.Task do
  use Ecto.Schema
  import Ecto.Changeset

  schema "tasks" do
    field :title, :string
    field :completed, :boolean, default: false
    field :description, :string
    field :status, :string, default: "pending"
    belongs_to :goal, TaskApi.Goal

    timestamps()
  end

  @doc false
  def changeset(task, attrs) do
    task
    |> cast(attrs, [:title, :completed, :description, :status, :goal_id])
    |> validate_required([:title, :status])
    |> validate_length(:title, min: 3)
    |> validate_inclusion(:status, ["pending", "in_progress", "completed"])
    |> foreign_key_constraint(:goal_id)
  end
end
