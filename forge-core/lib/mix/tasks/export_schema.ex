defmodule Mix.Tasks.ExportSchema do
  use Mix.Task

  @shortdoc "Exports the GraphQL schema to SDL"
  def run(_) do
    # Start the application to ensure all modules are loaded
    Mix.Task.run("app.start")

    # Generate the schema SDL (Schema Definition Language)
    sdl = Absinthe.Schema.to_sdl(TaskApiWeb.Schema)

    # Write to a file
    File.write!("schema.graphql", sdl)

    Mix.shell().info("Schema exported to schema.graphql")
  end
end
