defmodule TaskApi.Task do
  use Ecto.Schema
  import Ecto.Changeset

  schema "tasks" do
    field :title, :string
    field :completed, :boolean, default: false
    field :description, :string
    field :status, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(task, attrs) do
    task
    |> cast(attrs, [:title, :description, :status])
    |> validate_required([:title, :status])
    |> validate_length(:title, min: 3)
    |> validate_inclusion(:status, ["pending", "in_progress", "completed"])
  end
end
