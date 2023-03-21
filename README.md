# LibRustZcash UniFFI port

This is the Repository for the codebase concerning the [Zcash RFP](https://forum.zcashcommunity.com/t/rfp-zcash-uniffi-library/41335) published some time ago on the Forum. [Eiger](https://www.eiger.co/) won the grant to realize the RFP with [this proposal](https://forum.zcashcommunity.com/t/zcash-uniffi-library-rfp/43468).

⚠️ Production use is discouraged, as this project is still under heavy development.

## Available documentation

* We are currently using [Github milestones](https://github.com/eigerco/uniffi-zcash-lib/milestones).

* An updated feature matrix with the API surface status can be found [here](./STATUS.md).

* If you are new to the project, probably the [wiki](https://github.com/eigerco/uniffi-zcash-lib/wiki) is a good starting point.

## Contributors

 - [@eloylp](https://github.com/eloylp)
 - [@hauleth](https://github.com/hauleth)
 - [@MeerKatDev](https://github.com/MeerKatDev)
 - [@zduny](https://github.com/zduny)

## License

All code in this workspace is licensed under [MIT license](http://opensource.org/licenses/MIT).

## Conventions
 - We shall use Github Actions for the CI/CD, as long as it's maintainable and practical.
 - For local development there will be the option to use a Docker image provided by us. We should also maintain a section to help getting started for local tooling.
 - In order to double check compatibility, we shall use Test Vectors generated _indirectly_ by the librustzcash, whose data will be used to test the libraries for the foreign languages. This should ensure full compatibility.

## Generating documentation

At the moment the UniFFI library doesn't have the ability to add inline documentation. We are prototyping that feature [here](https://github.com/eigerco/uniffi-rs). We will fork the library and add the ability to do so, and a prototype should be available during the next milestone. The tools to generate documentation for each platform are:

 - Kotlin: [Dokka](https://kotlinlang.org/docs/kotlin-doc.html)
 - Python: [Sphynx]()
 - Ruby: [YARD]()
 - Swift: [DDoc]()

which are, to our understanding, the most common tools used in each ecosystem.

## Development procedure

At the moment we don't have a stable team on this, but once we have a team in place we should be able to define the development guidelines and stabilize the workflow (first two weeks of February).

# Local environment setup

## MacOS

### Easy, using Homebrew:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup update

brew install kotlin
brew install swift
brew install python
brew install ruby
```

## Linux

### (Ubuntu/Debian, APT package manager)

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

# Docker setup

There is a Docker image for development on emulated system, but at the moment only for M1 - although it's easily configurable for other architectures. We'll make a set of instructions available for it.

# Using the uniffi-bindgen CLI

This project has a dedicated crate for build and run the CLI. You can execute it from any crate root by:

```bash
cargo run -p uniffi-bindgen
```
