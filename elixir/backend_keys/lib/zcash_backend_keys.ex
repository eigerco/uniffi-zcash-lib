defmodule ZcashBackendKeys do
  use Rustler,
    otp_app: :zcash_backend_keys,
    crate: "nif_zcash_backend_keys"

  def add(_a, _b), do: :erlang.nif_error(:not_loaded)
end
