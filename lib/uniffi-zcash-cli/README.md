# Zcash UniFFI CLI 🕹️

An internal repo CLI to manage repo workflows, like binding generation, packaging and publishing.

## <a name="motivation"></a> Motivation

The UniFFI project [is not providing](https://mozilla.github.io/uniffi-rs/Motivation.html#why-not) a way to pack and publish the generated bindings. This CLI is an effort to provide a first version of a packaging/publishing solution. Some
of the key points are:

* Programmatically interact with all repo workflows. Abstract (but not hide) low level details of UniFFI tooling.
  
* Exponential backoff based retries for dealing with external services calls when publishing. Avoid partial releases as much as possible.
  
* Clear separation among building steps and the publishing ones.

* A multi-stage build and packaging process, that intends to help developers in debugging scenarios, but to also provide access to alternative ways of experimenting and importing the bindings.

* To decouple as much as possible from specific CI configurations (github CI, gitlab CI, etc ...) by concentrating all the logic in Rust code. 
  
* Written in Rust, a scalable language, with access to an entire ecosystem of libraries.

## <a name="how-to-execute"></a> How to execute

**Prerequisites**:  Ensure you have the [needed software](../../CONTRIBUTING.md#local-environment-setup) installed.

This CLI is only exposed as a binary crate. From [/lib](..) it can be executed with cargo by:

```bash
$ cargo run -p uniffi-zcash-cli -- --help
```

The low level `uniffi-bindgen` CLI can still be executed by:
```bash
cargo run -p uniffi-bindgen --help
```
But it will only provide low level binding generation operations.
## <a name="commands-and-design"></a> Commands and overall design

The CLI has the following build related available commands. They need to be executed in the following order:

```mermaid
graph LR;
bindgen-->release-->publish;
```

1. `sharedlibs` - It generates the C shared library the bindings need to import for both, MacOs and Linux. It leaves its output at `lib/shared_libs` . Its needed as pre-requisites... TBD. 

2. `bindgen` - It accepts a comma separated list of target `languages` . This command calls all the needed UniFFI machinery for generating each language bindings. It invokes the UniFFI tools under the hood, passing [our desired values](./../uniffi-bindgen/uniffi.toml) by default. The outcome of executing this command is a folder at `lib/bindings` , with a subfolder per each language that holds per language necessary files.

3. `release` - This command has a subcommand per each target language. It normally accepts a `version` argument among others (see help for more information). This command **doesn't push the artifacts yet**. It only prepares them by using a little, in house [project template system](./templates/). Such system has predefined projects structures for the different languages, which later are parametrized with a text template engine. It also copies the needed files from the previous command outcome at lib/bindings. The outcome of this command is placed at the `lib/packages` git ignored folder, with a subfolder per each language. It contains the packages ready to be published. This **packages are also automatically tested against little sample applications**. Such [sample applications](./templates/) just import the artifact as an user would do,  exercising the application code in way that checks that the entire import chain/dynamic library loading is not broken.

    At the end of this command execution, and per language, we should see something like:

    ```bash
    $ cargo run -p uniffi-zcash-cli release python --version 0.0.0
    ... more output ...
    Python test application successfully executed ✅
    ```

4. `publish` - This is the last step and only does the final publish operations i.e pushing previously generated artifacts at `lib/packages` . Its where most of the external calls are concentrated. As artifacts tend to be a bit weighty, it uses exponential backoff for pushing the artifacts to each language specific registry.

There are also other utility commands to help developers in testing stages:

`saplingparams` - Downloads the [sapling parameters](https://z.cash/technology/paramgen/) to the default location, normally in developers home directory.

`testdata` - Tests are based in inputs and outputs. This command regenerates the [golden data file](../uniffi-zcash/tests/test_data.csv) by executing the [data generators](../uniffi-zcash-test/src/test_data/).

## <a name="ci-integration"></a> Integration with the CI 🤖

The commented modular design allows many options for configuring the CI. Steps can be configured in parallel. It could be also possible to require some manual intervention before promoting the final publish steps. This will depend to each project needs. An example of use can be found on [this repo workflows folder](../../.github/workflows/).

## <a name="how-to-use"></a> How to use it from my laptop 💻  

1. Copy the [provided](./env_example) `env` file to a personal, git ignored `.env` one, and fill the variables values. By default, they have testing values. Only those for the relevant target languages are needed.
    ```bash
    $ cp ./env_example .env 
    ```
2. Execute the [script](./load_env.sh) for loading the needed environment variables into the current terminal:
    ```bash
    $ source ./load_env.sh
    ```
3. Now the developer can execute the desired commands.

## <a name="how-to-test"></a> How to test this CLI ✅

Currently the testing is done in a manual way. There is a [docker-compose.yml](./docker/docker-compose.yml) file with service mocks for the majority of the language registries that can be used to test the package publication. The chosen services are should be 100% compliant with the official registries.

The provided [env_example](./env_example) has the necessary values for interacting with the mock service registries provided in the [docker-compose.yml](./docker/docker-compose.yml) file. See [how-to-use](#how-to-use-it-from-my-laptop-💻) section for more details.

For most of the services that should be enough. How ever the are some specific nuances per language:

* In the case of `archiva` the service for maven packages, one needs to set a password for admin that later should be used in the commented env vars.
  
* In case of swift, only the Git repository publication can be tested at the moment. One needs a Git URL repo, and probably a personal access token included in that repo url following basic auth scheme. For github based repos, you can check [personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
