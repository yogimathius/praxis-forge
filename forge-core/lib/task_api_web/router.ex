defmodule TaskApiWeb.Router do
  use TaskApiWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  # Browser pipeline for LiveDashboard
  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  scope "/api", TaskApiWeb do
    pipe_through :api
    resources "/goals", GoalController, except: [:new, :edit]
    resources "/tasks", TaskController, except: [:new, :edit]
  end

  # LiveDashboard route
  if Mix.env() in [:dev] do
    import Phoenix.LiveDashboard.Router

    scope "/" do
      pipe_through :browser
      live_dashboard "/dashboard"
    end
  end
end
