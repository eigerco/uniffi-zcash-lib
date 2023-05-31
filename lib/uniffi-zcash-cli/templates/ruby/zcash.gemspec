Gem::Specification.new do |s|
    s.name        = "zcash"
    s.version     = "{{version}}"
    s.summary     = "The librustzcash ruby FFI binding"
    s.description = "A library for interacting with the librustzcash lib, a privacy oriented cryptocurrency"
    s.authors     = ["test"]
    s.email       = "test@test.com"
    # Here we require the ffi gem.
    s.add_runtime_dependency 'ffi', '1.15.5'
    # Here we require the so gem for dynamically loading the needed shared lib.
    s.add_runtime_dependency 'os', '1.1.4'                        
    # Adding the shared libraries and the bindings.
    s.files       = ["lib/zcash.rb", "lib/libuniffi_zcash.so", "lib/libuniffi_zcash.dylib"]      
    s.homepage    = "https://github.com/eigerco/uniffi-zcash-lib"
    s.license       = "MIT"
  end