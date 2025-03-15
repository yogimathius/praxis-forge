defmodule TaskApiWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :task_api
  require Logger

  # Add this before any other plugs
  plug CORSPlug,
    origin: ["http://localhost:1420", "http://localhost:3000", "http://127.0.0.1:3000", "http://127.0.0.1:1420"],
    methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
    headers: ["Content-Type", "Accept", "Authorization"],
    expose: ["Content-Type", "Authorization"],
    max_age: 86400,
    credentials: true

  # The session will be stored in the cookie and signed,
  # this means its contents can be read but not tampered with.
  # Set :encryption_salt if you would also like to encrypt it.
  @session_options [
    store: :cookie,
    key: "_task_api_key",
    signing_salt: "bUZiXROg",
    same_site: "Lax"
  ]

  socket "/live", Phoenix.LiveView.Socket,
    websocket: [connect_info: [session: @session_options]],
    longpoll: [connect_info: [session: @session_options]]

  # Serve at "/" the static files from "priv/static" directory.
  #
  # You should set gzip to true if you are running phx.digest
  # when deploying your static files in production.
  plug Plug.Static,
    at: "/",
    from: :task_api,
    gzip: false,
    only: TaskApiWeb.static_paths()

  # Code reloading can be explicitly enabled under the
  # :code_reloader configuration of your endpoint.
  if code_reloading? do
    plug Phoenix.CodeReloader
    plug Phoenix.Ecto.CheckRepoStatus, otp_app: :task_api
  end

  plug Phoenix.LiveDashboard.RequestLogger,
    param_key: "request_logger",
    cookie_key: "request_logger"

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Phoenix.json_library()

  plug Plug.MethodOverride
  plug Plug.Head
  plug Plug.Session, @session_options

  # Add the structured logging configuration here
  plug :log_request_info

  plug TaskApiWeb.Router

  # Add this function at the bottom of the module
  defp log_request_info(conn, _opts) do
    start_time = System.monotonic_time()

    Plug.Conn.register_before_send(conn, fn conn ->
      stop_time = System.monotonic_time()
      duration = System.convert_time_unit(stop_time - start_time, :native, :millisecond)

      Logger.info("Request processed",
        method: conn.method,
        path: conn.request_path,
        status: conn.status,
        duration_ms: duration,
        request_id: conn.assigns[:request_id] || "unknown"
      )

      conn
    end)
  end
end
