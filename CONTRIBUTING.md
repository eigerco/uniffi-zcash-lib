# Contributing

Welcome ! and thanks for landing here. We are currently in the first stages of this project. Your contributions, in any point of the wide spectrum
ideas, opinions, code .. Are more than welcome.

We just must follow this initially established (but not fixed) conventions.

## <a href="conventions"></a> Conventions

* We have defined [some conventions](../../wiki/Bindings-source-code-conventions) for dealing with the rough edges of our Rust wrapper code. They must be followed,
  unless new agreements are met.
* Keep all repository workflows in the internal [CLI](./lib/uniffi-zcash-cli/README.md).
* We use Github Actions for the CI/CD, as long as it's maintainable and practical. We try to avoid putting lots of logic there by moving it to our  [CLI](./lib/uniffi-zcash-cli/README.md).
* All knowledge should be added to our [wiki](../../wiki) and maintained.
* No functionality is implemented, only translated from the [librustzcash](https://github.com/zcash/librustzcash) library. As such, we may only accept contributions that respect this principle.

## <a href="development-procedure"></a> Development procedure

* Most code contributions must be in form of PRs originated in contributors repository forks.
* Small, direct PRs are encouraged.
* If a BIG idea is around, please, its better to create a [new issue](../../issues/new) first and ask for feedback.

## <a href="repo-cli"></a> Using the repo CLI

This CLI allows to easy build and publish the bindings and packages. We encourage to read the in crate [documentation](./lib/uniffi-zcash-cli/README.md).

## <a href="local-environment-setup"></a> Local environment setup

### <a href="local-environment-macos"></a> MacOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup update

brew install kotlin
brew install kylef/formulae/swiftenv
brew install python
brew install ruby
brew install wget
```

Use `swiftenv` to setup a system wide swift version.

In case of problems, re-check:

* Ensure ruby has an user writable path for storing gems. Sometimes this needs to be setup by:
  ```
  export GEM_HOME="$HOME/.gem"
  ```
* By default, you could be using the default system ruby installation (check with `which ruby` ) if not pointing to homebrew, this needs to be setup:
  ```
  export PATH="/opt/homebrew/opt/ruby/bin:$PATH"
  ```
* Ensure x-code tools are properly installed with developer tools: `xcode-select --install`.
* Ensure your `SDKROOT` environment variable is configured.

### <a href="local-environment-linux"></a> Linux

```bash
# Kotlin
sudo apt install default-jdk # if you still don't have Java installed
curl -s https://get.sdkman.io | bash # restart the terminal after this
sdk install kotlin

# Ruby
sudo apt-get install ruby-full

# Swift
# Download, un-tar, put in the path the binary from (here) <https://www.swift.org/download>.

# Python
sudo apt-get install python3.x # 3.8, 3.9
```

### <a href="local-environment-docker"></a> Docker setup

There is a [Docker image](./docker/Dockerfile) for development on emulated system, but at the moment only for M1 - although it's easily configurable for other architectures. We'll make a set of instructions available for it.

## <a href="testing"></a> Testing

All the tests are located in the [testing](./lib/uniffi-zcash/tests/) directory. 

### <a href="executing-tests"></a> Executing the tests

Ensure the `Sapling` crypto parameters are downloaded. You can download them
to your `$home` with the in repo [CLI](./lib/uniffi-zcash-cli/README.md) by:

```bash
$ cargo run -p uniffi-zcash-cli saplingparams
```

Then the tests can be run by:

```bash
$ cargo test
```

### <a href="data-driven-tests"></a> Data driven tests

Having a different dataset per each language would make no sense since we are testing exactly the same functionality on each language. It would be hard to maintain. In order to address this, we developed [data generators](./lib/uniffi-zcash-test/src/test_data/) and a common [data key value store](./lib/uniffi-zcash-test/src/test_support.rs) that can be used from all the languages.

#### <a href="test-data-generation"></a> Test data generation

A [CSV file](./lib/uniffi-zcash/tests/test_data.csv) with test data is committed in the repo. In case a new test is added or a modification is done in any of them, the new data should be committed.

If its needed to add new data generators or modifying the actual golden file, please refer to `testdata` data generation CLI commands.
More details available in the [CLI docs](./lib/uniffi-zcash-cli/README.md)