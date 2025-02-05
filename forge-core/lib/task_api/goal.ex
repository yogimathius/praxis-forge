defmodule TaskApi.Goal do
  use Ecto.Schema
  import Ecto.Changeset

  schema "goals" do
    field :title, :string
    field :description, :string
    field :tasks_required, :integer
    field :tasks_completed, :integer, default: 0

    has_many :tasks, TaskApi.Task  # Establish the relationship

    timestamps()
  end

  def changeset(goal, attrs) do
    goal
    |> cast(attrs, [:title, :description, :tasks_required, :tasks_completed])
    |> validate_required([:title, :tasks_required])
    |> validate_number(:tasks_required, greater_than: 0)
  end
end
