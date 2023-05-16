Gem::Specification.new do |s|
    s.name        = "zcash"
    s.version     = "{version}"
    s.summary     = "The librustzcash ruby FFI binding"
    s.description = "A library for interacting with the librustzcash lib, a privacy oriented cryptocurrency"
    s.authors     = ["test"]
    s.email       = "test@test.com"
    s.add_runtime_dependency 'ffi', '1.15.5'                        # Here we require the ffi gem.
    s.files       = ["lib/zcash.rb", "lib/libuniffi_zcash.so"]      # Adding the shared library and the bindings.
    s.homepage    = "https://github.com/eigerco/uniffi-zcash-lib"
    s.license       = "MIT"
  end