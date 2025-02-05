defmodule TaskApi.Repo.Migrations.CreateGoals do
  use Ecto.Migration

  def change do
    create table(:goals) do
      add :title, :string
      add :description, :string
      add :tasks_required, :integer
      add :tasks_completed, :integer, default: 0

      timestamps()
    end

    # Add a foreign key to the tasks table
    alter table(:tasks) do
      add :goal_id, references(:goals, on_delete: :delete_all)  # Foreign key association
    end
  end
end
