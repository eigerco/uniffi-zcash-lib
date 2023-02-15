defmodule ZcashBackendKeys do
  use Rustler,
    otp_app: :zcash_backend_keys,
    crate: "zcash-backend-keys",
    path: "../../lib/uniffi_backend_keys",
    features: ["rustler"],
    load_from: {:zcash_backend_keys, "priv/native/libuniffi_backend_keys"}

  def from_bytes(_data), do: :erlang.nif_error(:not_loaded)
  def from_seed(_seed, _account_id), do: :erlang.nif_error(:not_loaded)

  def to_account_pubkey(_priv_key), do: :erlang.nif_error(:not_loaded)
end
