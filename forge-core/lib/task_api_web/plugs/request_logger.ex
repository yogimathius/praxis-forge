defmodule TaskApiWeb.Plugs.RequestLogger do
  require Logger

  def init(opts), do: opts

  def call(conn, _opts) do
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
