defmodule ZcashBackendKeys do
  use Rustler,
    otp_app: :zcash_backend_keys,
    crate: "zcash-backend-keys",
    path: "../../lib/uniffi_backend_keys",
    features: ["beam"],
    load_from: {:zcash_backend_keys, "priv/native/libuniffi_backend_keys"}

  import Record

  defrecordp(:priv_key, :zcash_priv_key, ref: nil)
  defrecordp(:pub_key, :zcash_pub_key, ref: nil)

  @opaque priv_key() :: record(:priv_key, ref: reference())
  @opaque pub_key() :: record(:pub_key, ref: reference())

  @spec from_bytes(<<_::32, _::_*8>>) :: priv_key()
  def from_bytes(_data), do: :erlang.nif_error(:not_loaded)

  @spec from_seed(<<_::32, _::_*8>>, pos_integer()) :: priv_key()
  def from_seed(_seed, _account_id), do: :erlang.nif_error(:not_loaded)

  def test_from_seed(_seed, _account_id), do: :erlang.nif_error(:not_loaded)

  @spec to_account_pubkey(priv_key()) :: pub_key()
  def to_account_pubkey(_priv_key), do: :erlang.nif_error(:not_loaded)
end
