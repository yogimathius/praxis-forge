defmodule TaskApi.Repo.Migrations.AddDescriptionAndStatusToTasks do
  use Ecto.Migration

  def change do
    alter table(:tasks) do
      add :description, :string
      add :status, :string, default: "pending"
    end
  end
end
