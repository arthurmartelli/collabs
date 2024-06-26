import gleam/erlang/process
import mist
import wisp

pub fn main() {
  wisp.configure_logger()

  let assert Ok(_) =
    wisp.mist_handler(fn(_) { todo }, "secret_key")
    |> mist.new
    |> mist.port(8000)
    |> mist.start_http

  process.sleep_forever()
}
