defmodule TaskApiWeb.Schema.Types do
  use Absinthe.Schema.Notation

  object :task do
    field :id, :id
    field :title, :string
    field :description, :string
    field :completed, :boolean
    field :status, :string
    field :goal, :goal
  end

  object :goal do
    field :id, :id
    field :title, :string
    field :description, :string
    field :tasks_required, :integer
    field :tasks_completed, :integer
    field :tasks, list_of(:task)
  end
end
